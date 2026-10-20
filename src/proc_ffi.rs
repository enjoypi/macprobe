use crate::config::{LineageConfig, PriorityConfig};
use crate::dl::Library;
use crate::error::ProbeError;
use std::ffi::c_void;

type ResponsibleFn = unsafe extern "C" fn(i32) -> i32;

#[repr(C)]
struct ShortInfo {
    pid: u32,
    ppid: u32,
    tail: [u32; 14],
}

#[repr(C)]
struct ThreadInfo {
    times: [u64; 2],
    head: [i32; 5],
    curpri: i32,
    tail: [i32; 2],
    name: [u8; 64],
}

const _: () = assert!(std::mem::size_of::<ThreadInfo>() == 112);

unsafe extern "C" {
    fn proc_pidinfo(pid: i32, flavor: i32, arg: u64, buffer: *mut c_void, size: i32) -> i32;
    fn proc_pidpath(pid: i32, buffer: *mut c_void, size: u32) -> i32;
}

pub fn responsible(cfg: &LineageConfig, pid: i32) -> Result<i32, ProbeError> {
    let lib = Library::open(&cfg.library, cfg.dlopen_mode)?;
    let sym = lib.symbol(&cfg.symbol)?;
    let f = unsafe { std::mem::transmute::<*mut c_void, ResponsibleFn>(sym) };
    let r = unsafe { f(pid) };
    if r < 0 {
        return Err(ProbeError::ResponsibleFailed);
    }
    Ok(r)
}

fn pidinfo<T>(pid: i32, flavor: i32, arg: u64) -> Option<T> {
    let mut out = unsafe { std::mem::zeroed::<T>() };
    let size = std::mem::size_of::<T>() as i32;
    let n = unsafe { proc_pidinfo(pid, flavor, arg, (&mut out as *mut T).cast(), size) };
    (n == size).then_some(out)
}

pub fn parent(cfg: &LineageConfig, pid: i32) -> Result<i32, ProbeError> {
    pidinfo::<ShortInfo>(pid, cfg.shortbsdinfo_flavor, 0)
        .map(|i| i.ppid as i32)
        .ok_or(ProbeError::ProcInfoFailed)
}

fn threads(cfg: &PriorityConfig, pid: i32) -> Result<Vec<u64>, ProbeError> {
    let mut ids = vec![0u64; cfg.max_threads];
    let size = std::mem::size_of_val(ids.as_slice()) as i32;
    let n = unsafe {
        proc_pidinfo(
            pid,
            cfg.listthreads_flavor,
            0,
            ids.as_mut_ptr().cast(),
            size,
        )
    };
    if n <= 0 {
        return Err(ProbeError::ProcInfoFailed);
    }
    if n == size {
        return Err(ProbeError::ThreadsTruncated);
    }
    ids.truncate(n as usize / std::mem::size_of::<u64>());
    Ok(ids)
}

pub fn priority(cfg: &PriorityConfig, pid: i32) -> Result<i32, ProbeError> {
    threads(cfg, pid)?
        .into_iter()
        .filter_map(|t| pidinfo::<ThreadInfo>(pid, cfg.threadinfo_flavor, t))
        .map(|i| i.curpri)
        .max()
        .ok_or(ProbeError::ProcInfoFailed)
}

pub fn path(cfg: &LineageConfig, pid: i32) -> Result<String, ProbeError> {
    let mut buf = vec![0u8; cfg.path_max];
    let n = unsafe { proc_pidpath(pid, buf.as_mut_ptr().cast(), buf.len() as u32) };
    if n <= 0 {
        return Err(ProbeError::PathFailed);
    }
    buf.truncate(n as usize);
    Ok(String::from_utf8_lossy(&buf).into_owned())
}

#[cfg(test)]
#[path = "proc_ffi_test.rs"]
mod tests;

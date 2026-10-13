use crate::config::LineageConfig;
use crate::dl::Library;
use crate::error::ProbeError;
use std::ffi::c_void;

type ResponsibleFn = unsafe extern "C" fn(i32) -> i32;

#[repr(C)]
struct BsdInfo {
    head: [u32; 4],
    ppid: u32,
    tail: [u32; 29],
}

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

pub fn parent(cfg: &LineageConfig, pid: i32) -> Result<i32, ProbeError> {
    let mut info = BsdInfo {
        head: [0; 4],
        ppid: 0,
        tail: [0; 29],
    };
    let size = std::mem::size_of::<BsdInfo>() as i32;
    let buf = (&mut info as *mut BsdInfo).cast();
    let n = unsafe { proc_pidinfo(pid, cfg.bsdinfo_flavor, 0, buf, size) };
    if n != size {
        return Err(ProbeError::ProcInfoFailed);
    }
    Ok(info.ppid as i32)
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

use crate::config::TccConfig;
use crate::dl::Library;
use crate::error::ProbeError;
use std::ffi::c_void;

type CfTypeRef = *const c_void;
type PreflightFn = unsafe extern "C" fn(CfTypeRef, CfTypeRef) -> i32;

pub fn preflight(cfg: &TccConfig) -> Result<i32, ProbeError> {
    let lib = Library::open(&cfg.framework, cfg.dlopen_mode)?;
    let sym = lib.symbol(&cfg.symbol)?;
    let service = unsafe { *(lib.symbol(&cfg.service)? as *const CfTypeRef) };
    let f = unsafe { std::mem::transmute::<*mut c_void, PreflightFn>(sym) };
    Ok(unsafe { f(service, std::ptr::null()) })
}

#[cfg(test)]
#[path = "tcc_ffi_test.rs"]
mod tests;

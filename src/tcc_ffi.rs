use crate::config::TccConfig;
use crate::dl::{Library, c_string};
use crate::error::ProbeError;
use std::ffi::{c_char, c_void};

type CfTypeRef = *const c_void;
type PreflightFn = unsafe extern "C" fn(CfTypeRef, CfTypeRef) -> i32;

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFStringCreateWithCString(alloc: CfTypeRef, s: *const c_char, encoding: u32) -> CfTypeRef;
    fn CFRelease(cf: CfTypeRef);
}

struct CfString(CfTypeRef);

impl CfString {
    fn new(s: &str, encoding: u32) -> Result<Self, ProbeError> {
        let s = c_string(s)?;
        let cf = unsafe { CFStringCreateWithCString(std::ptr::null(), s.as_ptr(), encoding) };
        if cf.is_null() {
            return Err(ProbeError::CfStringFailed);
        }
        Ok(CfString(cf))
    }
}

impl Drop for CfString {
    fn drop(&mut self) {
        unsafe { CFRelease(self.0) };
    }
}

pub fn preflight(cfg: &TccConfig) -> Result<i32, ProbeError> {
    let lib = Library::open(&cfg.framework, cfg.dlopen_mode)?;
    let sym = lib.symbol(&cfg.symbol)?;
    let service = CfString::new(&cfg.service, cfg.cf_encoding)?;
    let f = unsafe { std::mem::transmute::<*mut c_void, PreflightFn>(sym) };
    Ok(unsafe { f(service.0, std::ptr::null()) })
}

#[cfg(test)]
#[path = "tcc_ffi_test.rs"]
mod tests;

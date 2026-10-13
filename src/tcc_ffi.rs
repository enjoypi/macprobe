use crate::config::TccConfig;
use crate::error::ProbeError;
use std::ffi::{CString, c_char, c_void};

type CfTypeRef = *const c_void;
type PreflightFn = unsafe extern "C" fn(CfTypeRef, CfTypeRef) -> i32;

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> i32;
}

#[link(name = "CoreFoundation", kind = "framework")]
unsafe extern "C" {
    fn CFStringCreateWithCString(alloc: CfTypeRef, s: *const c_char, encoding: u32) -> CfTypeRef;
    fn CFRelease(cf: CfTypeRef);
}

fn c_string(s: &str) -> Result<CString, ProbeError> {
    CString::new(s).map_err(|_| ProbeError::InvalidCString)
}

struct Library(*mut c_void);

impl Library {
    fn open(path: &str, mode: i32) -> Result<Self, ProbeError> {
        let path = c_string(path)?;
        let handle = unsafe { dlopen(path.as_ptr(), mode) };
        if handle.is_null() {
            return Err(ProbeError::LoadFailed);
        }
        Ok(Library(handle))
    }

    fn symbol(&self, name: &str) -> Result<*mut c_void, ProbeError> {
        let name = c_string(name)?;
        let sym = unsafe { dlsym(self.0, name.as_ptr()) };
        if sym.is_null() {
            return Err(ProbeError::SymbolMissing);
        }
        Ok(sym)
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        unsafe { dlclose(self.0) };
    }
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

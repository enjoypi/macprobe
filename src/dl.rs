use crate::error::ProbeError;
use std::ffi::{CString, c_char, c_void};

unsafe extern "C" {
    fn dlopen(path: *const c_char, mode: i32) -> *mut c_void;
    fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    fn dlclose(handle: *mut c_void) -> i32;
}

pub fn c_string(s: &str) -> Result<CString, ProbeError> {
    CString::new(s).map_err(|_| ProbeError::InvalidCString)
}

pub struct Library(*mut c_void);

impl Library {
    pub fn open(path: &str, mode: i32) -> Result<Self, ProbeError> {
        let path = c_string(path)?;
        let handle = unsafe { dlopen(path.as_ptr(), mode) };
        if handle.is_null() {
            return Err(ProbeError::LoadFailed);
        }
        Ok(Library(handle))
    }

    pub fn symbol(&self, name: &str) -> Result<*mut c_void, ProbeError> {
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

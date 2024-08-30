use std::borrow::Cow;
use std::ffi::{c_char, c_void, CStr, CString};

use crate::cache::{compile_regex, RegexObject};

pub mod meta;
pub mod regex;

unsafe fn get_string<'a>(ptr: *const c_char) -> Cow<'a, str> {
    let c_str = CStr::from_ptr(ptr);
    c_str.to_string_lossy()
}

unsafe fn to_string(str: String) -> *const c_char {
    let c_str = CString::from_vec_unchecked(str.into_bytes());
    c_str.into_raw()
}

#[no_mangle]
unsafe extern "C" fn free_string(ptr: *const c_char) {
    if ptr.is_null() { return; }
    let _ = CString::from_raw(ptr as *mut c_char);
}

#[repr(C)]
struct RegexCompileResult {
    re: *const c_void,
    error: *const c_char,
}

#[no_mangle]
unsafe extern "C" fn rust_regex_compile(re: *const c_char) -> *const RegexCompileResult {
    let re = CStr::from_ptr(re).to_string_lossy();
    let re = match compile_regex(&re) {
        Ok(re) => RegexCompileResult {
            re: Box::into_raw(Box::new(re)) as *const c_void,
            error: std::ptr::null(),
        },
        Err(err) => RegexCompileResult {
            re: std::ptr::null(),
            error: to_string(err.to_string()),
        },
    };
    Box::into_raw(Box::new(re))
}

#[no_mangle]
unsafe extern "C" fn rust_regex_free_compile(ptr: *const RegexCompileResult) {
    if ptr.is_null() { return; }
    let re = Box::from_raw(ptr as *mut RegexCompileResult);
    if !re.re.is_null() {
        let _ = Box::from_raw(re.re as *mut RegexObject);
    }
    if !re.error.is_null() {
        free_string(re.error);
    }
}

unsafe fn get_regex<'a>(ptr: *const c_void) -> &'a RegexObject {
    &*(ptr as *const RegexObject)
}

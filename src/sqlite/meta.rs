use std::ffi::c_char;

use crate::regex::meta;
use crate::sqlite::to_string;

#[no_mangle]
unsafe extern "C" fn rust_regex_version() -> *const c_char {
    let version = meta::regex_version();
    to_string(version)
}

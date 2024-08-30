use std::ffi::{c_char, c_void};

use crate::regex::regex;
use crate::sqlite::{get_regex, get_string};

#[no_mangle]
unsafe extern "C" fn regex_is_match(re: *const c_void, haystack: *const c_char) -> bool {
    let re = get_regex(re);
    let haystack = get_string(haystack);
    regex::regex_is_match(&re, &haystack)
}
 
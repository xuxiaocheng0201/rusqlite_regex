//! Handle rusqlite errors.

use std::ffi::{c_int, CStr};

use rusqlite::{ffi, Error, Result};

/// This is a re-exported and enhanced version of [`rusqlite::error::check(res)`](rusqlite::error::check)
#[doc(hidden)]
#[allow(unsafe_code)]
pub fn check_err(res: c_int) -> Result<()> {
    if res == ffi::SQLITE_OK {
        return Ok(());
    }
    let err = unsafe { ffi::sqlite3_errstr(res) };
    if err.is_null() {
        return Err(Error::SqliteFailure(ffi::Error::new(res), None));
    }
    let msg = unsafe { CStr::from_ptr(err) }.to_str()?;
    Err(Error::SqliteFailure(ffi::Error::new(res), Some(msg.to_string())))
}

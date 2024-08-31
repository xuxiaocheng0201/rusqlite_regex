use std::ffi::{c_char, c_int};

use rusqlite::ffi;
use rusqlite::functions::FunctionFlags;

pub mod meta;
pub mod regex;

pub unsafe extern "C" fn sqlite3_regex_init(db: *mut ffi::sqlite3, pz_err_msg: *mut *mut c_char, _p_api: *const ffi::sqlite3_api_routines) -> c_int {
    rusqlite::auto_extension::init_auto_extension(db, pz_err_msg, |connection| {
        macro_rules! scalar_function {
            ($conn: ident += $module: ident::$function: ident($n: literal)) => {
                $conn.create_scalar_function(
                    stringify!($function),
                    $n,
                    FunctionFlags::SQLITE_UTF8 | FunctionFlags::SQLITE_DETERMINISTIC,
                    $module::$function
                )
            };
        }

        scalar_function!(connection += meta::regex_version(0))?;

        scalar_function!(connection += regex::regex_is_match(2))?;

        Ok(())
    })
}

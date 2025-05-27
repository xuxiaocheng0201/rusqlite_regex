#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(unsafe_code, missing_docs)]

pub mod cache;
pub mod extension;
pub mod error;

/// Enable sqlite3_regex_init() as an auto extension.
pub fn enable_auto_extension() -> rusqlite::Result<()> {
    #[allow(unsafe_code)]
    let res = unsafe { rusqlite::ffi::sqlite3_auto_extension(Some(extension::sqlite3_regex_init)) };
    error::check_err(res)
}

/// Disable sqlite3_regex_init() as an auto extension.
pub fn disable_auto_extension() -> rusqlite::Result<()> {
    #[allow(unsafe_code)]
    let res = unsafe { rusqlite::ffi::sqlite3_cancel_auto_extension(Some(extension::sqlite3_regex_init)) };
    error::check_err(res)
}

#[cfg(test)]
mod tester {
    pub fn initialize() -> Result<rusqlite::Connection, rusqlite::Error> {
        static ONCE: once_cell::sync::OnceCell<()> = once_cell::sync::OnceCell::new();
        ONCE.get_or_try_init(crate::enable_auto_extension)?;
        rusqlite::Connection::open_in_memory()
    }

    pub fn query<T: rusqlite::types::FromSql>(conn: &rusqlite::Connection, sql: &str) -> rusqlite::Result<T> {
        conn.query_row(sql, [], |row| row.get::<_, T>(0))
    }
}

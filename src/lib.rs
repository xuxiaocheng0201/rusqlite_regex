#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(unsafe_code, missing_docs)]

mod cache;
pub mod extension;

/// Enable sqlite3_regex_init() as an auto extension.
pub fn enable_auto_extension() -> rusqlite::Result<()> {
    #[allow(unsafe_code)]
    unsafe { rusqlite::auto_extension::register_auto_extension(extension::sqlite3_regex_init) }
}

/// Disable sqlite3_regex_init() as an auto extension.
pub fn disable_auto_extension() {
    rusqlite::auto_extension::cancel_auto_extension(extension::sqlite3_regex_init);
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

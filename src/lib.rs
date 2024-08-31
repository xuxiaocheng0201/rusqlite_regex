mod cache;
mod regex;

#[cfg(test)]
mod tester {
    pub fn initialize() -> Result<rusqlite::Connection, rusqlite::Error> {
        static ONCE: once_cell::sync::OnceCell<()> = once_cell::sync::OnceCell::new();
        ONCE.get_or_try_init(|| unsafe {
            rusqlite::auto_extension::register_auto_extension(crate::regex::sqlite3_regex_init)
        })?;
        rusqlite::Connection::open_in_memory()
    }
}

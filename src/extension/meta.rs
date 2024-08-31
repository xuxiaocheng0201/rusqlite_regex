//! Meta functions.

use rusqlite::functions::Context;

/// Return the version of current regex extension.
///
/// # Usage
/// ```sql
/// regex_version()
/// ```
#[inline]
pub fn regex_version(_context: &Context) -> Result<String, rusqlite::Error> {
    Ok(format!("v{}", env!("CARGO_PKG_VERSION")))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_regex_version() -> Result<(), rusqlite::Error> {
        let conn = crate::tester::initialize()?;
        assert_eq!(format!("v{}", env!("CARGO_PKG_VERSION")),
           conn.query_row(
               "SELECT regex_version()",
               [], |row| row.get::<_, String>(0)
           )?
        );
        Ok(())
    }
}

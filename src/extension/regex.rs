//! Regex functions.

use rusqlite::functions::Context;

use crate::cache::compile_regex;

/// [regex::Regex::is_match]
///
/// # Usage
/// ```sql
/// regex_is_match(pattern, haystack)
/// ```
#[inline]
pub fn regex_is_match(context: &Context) -> Result<bool, rusqlite::Error> {
    let re = compile_regex(&context.get::<String>(0)?)?;
    let haystack= context.get::<String>(1)?;
    Ok(re.is_match(&haystack))
}

/// Alias of [regex_is_match].
///
/// # Usage
/// ```sql
/// regexp(pattern, haystack)
/// ```
/// or
/// ```sql
/// haystack REGEXP pattern
/// ```
#[inline]
pub fn regexp(context: &Context) -> Result<bool, rusqlite::Error> {
    regex_is_match(context)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_regex_is_match() -> Result<(), rusqlite::Error> {
        let conn = crate::tester::initialize()?;
        conn.execute_batch("
            CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);
            INSERT INTO test (name) VALUES ('I categorically deny having triskaidekaphobia.');
        ")?;
        assert_eq!(1, crate::tester::query::<i32>(&conn, "SELECT id FROM test WHERE regex_is_match('\\b\\w{13}\\b', name)")?);
        assert_eq!(1, crate::tester::query::<i32>(&conn, "SELECT id FROM test WHERE name REGEXP '\\b\\w{13}\\b'")?);
        Ok(())
    }
}

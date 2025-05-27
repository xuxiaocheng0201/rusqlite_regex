//! Regex functions.

use std::borrow::Cow;

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
    let re = compile_regex(context.get(0)?)?;
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


/// [regex::Regex::replace]
///
/// # Usage
/// ```sql
/// regex_replace(pattern, haystack, replacer)
/// ```
#[inline]
pub fn regex_replace(context: &Context) -> Result<String, rusqlite::Error> {
    let re = compile_regex(context.get(0)?)?;
    let haystack= context.get::<String>(1)?;
    let replacer = context.get::<String>(2)?;
    match re.replace(&haystack, &replacer) {
        Cow::Owned(s) => Ok(s),
        Cow::Borrowed(_) => Ok(haystack), // keep the original string if no match
    }
}

/// [regex::Regex::replace_all]
///
/// # Usage
/// ```sql
/// regex_replace_all(pattern, haystack, replacer)
/// ```
#[inline]
pub fn regex_replace_all(context: &Context) -> Result<String, rusqlite::Error> {
    let re = compile_regex(context.get(0)?)?;
    let haystack= context.get::<String>(1)?;
    let replacer = context.get::<String>(2)?;
    match re.replace_all(&haystack, &replacer) {
        Cow::Owned(s) => Ok(s),
        Cow::Borrowed(_) => Ok(haystack), // keep the original string if no match
    }
}

/// [regex::Regex::replacen]
///
/// # Usage
/// ```sql
/// regex_replacen(pattern, haystack, limit, replacer)
/// ```
#[inline]
pub fn regex_replacen(context: &Context) -> Result<String, rusqlite::Error> {
    let re = compile_regex(context.get(0)?)?;
    let haystack= context.get::<String>(1)?;
    let limit = context.get::<usize>(2)?;
    let replacer = context.get::<String>(3)?;
    match re.replacen(&haystack, limit, &replacer) {
        Cow::Owned(s) => Ok(s),
        Cow::Borrowed(_) => Ok(haystack), // keep original string if no match
    }
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

    #[test]
    fn test_regex_replace() -> Result<(), rusqlite::Error> {
        let conn = crate::tester::initialize()?;
        conn.execute_batch("
            CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);
            INSERT INTO test (name) VALUES ('1078910');
        ")?;
        assert_eq!("1010", crate::tester::query::<String>(&conn, "SELECT regex_replace('[^01]+', name, '') FROM test")?);
        Ok(())
    }

    #[test]
    fn test_regex_replace_all() -> Result<(), rusqlite::Error> {
        let conn = crate::tester::initialize()?;
        conn.execute_batch("
            CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);
            INSERT INTO test (name) VALUES ('Greetings  1973\nWild\t1973\nBornToRun\t\t\t\t1975\nDarkness                    1978\nTheRiver 1980\n');
        ")?;
        assert_eq!("1973 Greetings\n1973 Wild\n1975 BornToRun\n1978 Darkness\n1980 TheRiver\n", crate::tester::query::<String>(&conn, "SELECT regex_replace_all('(?m)^(\\S+)[\\s--\\r\\n]+(\\S+)$', name, '$2 $1') FROM test")?);
        Ok(())
    }

    #[test]
    fn test_regex_replacen() -> Result<(), rusqlite::Error> {
        let conn = crate::tester::initialize()?;
        conn.execute_batch("
            CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);
            INSERT INTO test (name) VALUES ('Greetings  1973\nWild\t1973\nBornToRun\t\t\t\t1975\nDarkness                    1978\nTheRiver 1980\n');
        ")?;
        assert_eq!("1973 Greetings\n1973 Wild\nBornToRun\t\t\t\t1975\nDarkness                    1978\nTheRiver 1980\n", crate::tester::query::<String>(&conn, "SELECT regex_replacen('(?m)^(\\S+)[\\s--\\r\\n]+(\\S+)$', name, 2, '$2 $1') FROM test")?);
        Ok(())
    }
}

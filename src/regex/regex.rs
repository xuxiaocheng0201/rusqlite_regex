use rusqlite::functions::Context;

use crate::cache::compile_regex;

#[inline]
pub fn regex_is_match(context: &Context) -> Result<bool, rusqlite::Error> {
    let re = compile_regex(&context.get::<String>(0)?)?;
    let haystack= context.get::<String>(1)?;
    Ok(re.is_match(&haystack))
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
        assert_eq!(1,
           conn.query_row(
               "SELECT id FROM test WHERE regex_is_match('\\b\\w{13}\\b', name)",
               [], |row| row.get::<_, i64>(0)
           )?
        );
        Ok(())
    }
}

# rusqlite_regex

[![Crate](https://img.shields.io/crates/v/rusqlite_regex.svg)](https://crates.io/crates/rusqlite_regex)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/rusqlite_regex)](https://github.com/xuxiaocheng0201/rusqlite_regex/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/rusqlite_regex)](https://github.com/xuxiaocheng0201/rusqlite_regex/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/rusqlite_regex)](https://github.com/xuxiaocheng0201/rusqlite_regex/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/rusqlite_regex)](https://github.com/xuxiaocheng0201/rusqlite_regex/blob/master/LICENSE)


# Description

A SQLite extension for regular expressions written in pure Rust.
Based on [rusqlite](https://crates.io/crates/rusqlite) and [regex](https://crates.io/crates/regex).


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rusqlite_regex = "~0.1"
```


# Example

```rust
fn main() -> Result<(), rusqlite::Error> {
    rusqlite_regex::enable_auto_extension()?;
    
    let conn = rusqlite::Connection::open_in_memory()?;
    conn.execute_batch("
        CREATE TABLE test (id INTEGER PRIMARY KEY, name TEXT);
        INSERT INTO test (name) VALUES ('I categorically deny having triskaidekaphobia.');
    ")?;
    assert_eq!(1,
       conn.query_row(
           "SELECT id FROM test WHERE name REGEXP '\\b\\w{13}\\b'",
           [], |row| row.get::<_, i64>(0)
       )?
    );
    Ok(())
}
```


# License

This project is licensed under either of

Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.

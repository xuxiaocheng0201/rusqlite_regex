use std::ops::Deref;

use once_cell::sync::Lazy;
use regex::Regex;
use rusqlite::Error;

#[allow(dead_code)]
pub struct RegexCache();
#[allow(dead_code)]
static CACHE: Lazy<RegexCache> = Lazy::new(|| RegexCache {});

pub struct RegexObject {
    regex: Regex,
}

impl Deref for RegexObject {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.regex
    }
}


fn translate_error(err: regex::Error) -> Error {
    Error::UserFunctionError(Box::new(err) as _)
}

pub fn compile_regex(re: &str) -> Result<RegexObject, Error> {
    Ok(RegexObject { regex: Regex::new(re).map_err(translate_error)? })
}

use std::ops::Deref;

use once_cell::sync::Lazy;
use regex::Regex;

use crate::error::Result;

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


pub fn compile_regex(re: &str) -> Result<RegexObject> {
    Ok(RegexObject { regex: Regex::new(re)? })
}

//! Compiled regex cache.

pub use internal::*;

fn translate_error(err: regex::Error) -> rusqlite::Error {
    rusqlite::Error::UserFunctionError(Box::new(err) as _)
}

#[cfg(feature = "cache")]
mod internal {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    use once_cell::sync::Lazy;
    use quick_cache::sync::Cache;
    use regex::Regex;

    static CACHE_SIZE: AtomicUsize = AtomicUsize::new(32);

    static CACHE: Lazy<Cache<Arc<String>, Arc<Regex>>> = Lazy::new(|| Cache::new(CACHE_SIZE.load(Ordering::Acquire)));

    /// Set the regex cache size.
    ///
    /// You must call this function before any regexes are compiled.
    #[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
    pub fn set_cache_size(size: usize) {
        CACHE_SIZE.store(size, Ordering::Release)
    }

    /// Get the real regex cache size in used.
    ///
    /// This is the actual capacity of the cache, not the size set by [set_cache_size].
    #[cfg_attr(docsrs, doc(cfg(feature = "cache")))]
    pub fn get_cache_size() -> usize {
        match Lazy::get(&CACHE) {
            Some(cache) => cache.capacity() as usize,
            None => CACHE_SIZE.load(Ordering::Acquire),
        }
    }

    pub(crate) fn compile_regex(re: String) -> Result<Arc<Regex>, rusqlite::Error> {
        let re = Arc::new(re);
        CACHE.get_or_insert_with(&re, || Regex::new(&re).map(Arc::new)).map_err(super::translate_error)
    }
}

#[cfg(not(feature = "cache"))]
mod internal {
    use std::sync::Arc;

    use regex::Regex;

    pub(crate) fn compile_regex(re: String) -> Result<Arc<Regex>, rusqlite::Error> {
        Ok(Arc::new(Regex::new(&re).map_err(super::translate_error)))
    }
}

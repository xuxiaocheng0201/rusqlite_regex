use regex::Regex;

#[inline]
pub fn regex_is_match(re: &Regex, haystack: &str) -> bool {
    re.is_match(haystack)
}

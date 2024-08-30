#[inline]
pub fn regex_version() -> String {
    format!("v{}", env!("CARGO_PKG_VERSION"))
}

use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Regex(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

use core::fmt;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, serde::Serialize)]
pub enum Error {
    FailToCreatePool(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

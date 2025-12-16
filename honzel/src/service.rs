use std::error::Error as StdError;
use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Storage(sqlx::Error),
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::Storage(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Storage(err) => format!("storage error: {err}"),
        };
        write!(f, "{s}")
    }
}

impl StdError for Error {}

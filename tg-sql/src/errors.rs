use rusqlite::Error;
use std::fmt;

#[derive(Debug)]
pub enum StoreError{
    InvalidKey(String),
    Database(Error)
}

//automatic map_err
impl From<Error> for StoreError {
    fn from(error: Error) -> Self {
        StoreError::Database(error)
    }
}

impl From<String> for StoreError {
    fn from(error: String) -> Self {
        StoreError::InvalidKey(error)
    }
}

impl fmt::Display for StoreError {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            StoreError::InvalidKey(message) => {
                write!(f, "Invalid key: {}", message)
            }
            StoreError::Database(error) => {
                write!(f, "Database error: {}", error)
            }
        }
    }
}

impl std::error::Error for StoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            StoreError::InvalidKey(_) => None,
            StoreError::Database(error) => Some(error),
        }
    }
}
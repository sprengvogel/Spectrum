use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum FileStoreError {
    IO(std::io::Error),
    Diesel(diesel::result::Error),
}

impl Display for FileStoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FileStoreError {}

impl From<std::io::Error> for FileStoreError {
    fn from(error: std::io::Error) -> Self {
        FileStoreError::IO(error)
    }
}

impl From<diesel::result::Error> for FileStoreError {
    fn from(error: diesel::result::Error) -> Self {
        FileStoreError::Diesel(error)
    }
}

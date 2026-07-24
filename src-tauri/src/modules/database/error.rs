use std::fmt;

#[derive(Debug)]
pub enum StorageError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    NotFound(String),
    Other(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StorageError::IoError(e) => write!(f, "IO error: {}", e),
            StorageError::SerdeError(e) => write!(f, "Serialization error: {}", e),
            StorageError::NotFound(s) => write!(f, "Not found: {}", s),
            StorageError::Other(s) => write!(f, "Storage error: {}", s),
        }
    }
}

impl std::error::Error for StorageError {}

impl From<std::io::Error> for StorageError {
    fn from(e: std::io::Error) -> Self {
        StorageError::IoError(e)
    }
}

impl From<serde_json::Error> for StorageError {
    fn from(e: serde_json::Error) -> Self {
        StorageError::SerdeError(e)
    }
}

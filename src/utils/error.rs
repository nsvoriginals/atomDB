use std::fmt;

#[derive(Debug)]
pub enum DatabaseError {
    TableNotFound(String),
    SerializationError(String),
    DeserializationError(String),
    InvalidQuery(String),
}

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DatabaseError::TableNotFound(name) => write!(f, "Table '{}' not found", name),
            DatabaseError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
            DatabaseError::DeserializationError(msg) => write!(f, "Deserialization error: {}", msg),
            DatabaseError::InvalidQuery(msg) => write!(f, "Invalid query: {}", msg),
        }
    }
}

impl std::error::Error for DatabaseError {}

use std::{fmt, error};

#[derive(Debug)]
pub enum CatalogError {
    DatabaseError(rusqlite::Error),
    AuthorNotFound,
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CatalogError::DatabaseError(..) =>
                write!(f, "there was an error in accessing the database"),
            CatalogError::AuthorNotFound =>
                write!(f, "author does not exist in catalog"),
        }
    }
}

impl error::Error for CatalogError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            CatalogError::DatabaseError(ref e) => Some(e),
            CatalogError::AuthorNotFound => None,
        }
    }
}
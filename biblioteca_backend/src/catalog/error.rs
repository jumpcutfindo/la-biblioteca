use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum CatalogError {
    DatabaseError(#[from] rusqlite::Error),
    AuthorNotFound,
}

impl fmt::Display for CatalogError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CatalogError::DatabaseError(..) => {
                write!(f, "there was an error in accessing the database")
            }
            CatalogError::AuthorNotFound => write!(f, "author does not exist in catalog"),
        }
    }
}

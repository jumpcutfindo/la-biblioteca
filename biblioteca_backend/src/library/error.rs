use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum LibraryError {
    DatabaseError(#[from] rusqlite::Error),
    ResourceNotExists,
    BookBorrowed,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibraryError::DatabaseError(..) =>
                write!(f, "there was an error in accessing the database"),
            LibraryError::ResourceNotExists =>
                write!(f, "either user or book does not exist"),
            LibraryError::BookBorrowed =>
                write!(f, "book has been borrowed"),
        }
    }
}
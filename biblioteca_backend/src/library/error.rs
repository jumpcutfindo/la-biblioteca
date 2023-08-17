use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum LibraryError {
    DatabaseError(#[from] rusqlite::Error),
    ResourceNotExists,
    BookAlreadyBorrowed,
    BookAlreadyReturned,
    BookNotBorrowedByUser,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibraryError::DatabaseError(..) =>
                write!(f, "there was an error in accessing the database"),
            LibraryError::ResourceNotExists =>
                write!(f, "either user or book does not exist"),
            LibraryError::BookAlreadyBorrowed =>
                write!(f, "book has already been borrowed"),
            LibraryError::BookAlreadyReturned =>
                write!(f, "book has already been returned"),
            LibraryError::BookNotBorrowedByUser =>
                write!(f, "book was not borrowed by given user"),
        }
    }
}
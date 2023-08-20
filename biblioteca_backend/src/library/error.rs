use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum LibraryError {
    DatabaseError(#[from] rusqlite::Error),
    UserNotExists,
    BookNotExists,
    ResourceNotExists,
    BookAlreadyBorrowed,
    BookAlreadyReturned,
    BookNotBorrowedByUser,
    NumBorrowableExceeded(u32),
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibraryError::DatabaseError(..) =>
                write!(f, "there was an error in accessing the database"),
            LibraryError::UserNotExists =>
                write!(f, "user does not exist"),
            LibraryError::BookNotExists => 
                write!(f, "book does not exist"),
            LibraryError::ResourceNotExists =>
                write!(f, "either user or book does not exist"),
            LibraryError::BookAlreadyBorrowed =>
                write!(f, "book has already been borrowed"),
            LibraryError::BookAlreadyReturned =>
                write!(f, "book has already been returned"),
            LibraryError::BookNotBorrowedByUser =>
                write!(f, "book was not borrowed by given user"),
            LibraryError::NumBorrowableExceeded(max) =>
                write!(f, "user has reached max num of borrowable books (max: {})", max),
        }
    }
}
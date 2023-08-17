use std::fmt;

#[derive(thiserror::Error, Debug)]
pub enum LibraryError {
    BookBorrowed,
}

impl fmt::Display for LibraryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LibraryError::BookBorrowed =>
                write!(f, "book has been borrowed"),
        }
    }
}
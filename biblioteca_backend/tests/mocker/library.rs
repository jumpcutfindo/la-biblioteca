use biblioteca_backend::library::model::{BookBorrowEntry, BookBorrowState};
use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct MockLibrary {}

pub struct MockBookBorrowEntryBuilder {
    id: Uuid,
    book_id: Uuid,
    user_id: Uuid,
    timestamp: DateTime<Utc>,
    state: BookBorrowState,
}

impl MockBookBorrowEntryBuilder {
    pub fn id(mut self, id: Uuid) -> MockBookBorrowEntryBuilder {
        self.id = id;
        self
    }

    pub fn book_id(mut self, book_id: Uuid) -> MockBookBorrowEntryBuilder {
        self.book_id = book_id;
        self
    }

    pub fn user_id(mut self, user_id: Uuid) -> MockBookBorrowEntryBuilder {
        self.user_id = user_id;
        self
    }

    pub fn timestamp(mut self, timestamp: DateTime<Utc>) -> MockBookBorrowEntryBuilder {
        self.timestamp = timestamp;
        self
    }

    pub fn state(mut self, state: BookBorrowState) -> MockBookBorrowEntryBuilder {
        self.state = state;
        self
    }

    pub fn build(self) -> BookBorrowEntry {
        BookBorrowEntry {
            id: self.id,
            book_id: self.book_id,
            user_id: self.user_id,
            timestamp: self.timestamp,
            state: self.state,
        }
    }
}

impl MockLibrary {
    pub fn new_borrow_entry() -> MockBookBorrowEntryBuilder {
        MockBookBorrowEntryBuilder {
            id: Uuid::new_v4(),
            book_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            state: BookBorrowState::Borrowed,
        }
    }

    pub fn new_return_entry() -> MockBookBorrowEntryBuilder {
        MockBookBorrowEntryBuilder {
            id: Uuid::new_v4(),
            book_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            timestamp: Utc::now(),
            state: BookBorrowState::Returned,
        }
    }
}

use std::fs::remove_file;

use biblioteca_backend::{
    catalog::model::{Author, Book},
    database::setup_db, users::model::User, library::model::BookBorrowState,
};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use uuid::Uuid;

pub struct MockDatabaseBuilder {
    pub connection: Pool<SqliteConnectionManager>,
}

impl MockDatabaseBuilder {
    pub fn create(database_path: String) -> MockDatabaseBuilder {
        return MockDatabaseBuilder {
            connection: setup_db(database_path).unwrap(),
        };
    }

    pub fn teardown(database_path: String) {
        remove_file(database_path).unwrap();
    }

    pub fn with_author(self, author: &Author) -> MockDatabaseBuilder {
        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO authors (id, name, description, country) VALUES (?1, ?2, ?3, ?4)",
                (
                    &author.id,
                    &author.name,
                    &author.description,
                    &author.country,
                ),
            )
            .unwrap();

        return self;
    }

    pub fn with_book(self, book: &Book, author_id: &Uuid) -> MockDatabaseBuilder {
        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO books (id, name, description, language) VALUES (?1, ?2, ?3, ?4)",
                (&book.id, &book.name, &book.description, &book.language),
            )
            .unwrap();

        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO map_books_to_authors (book_id, author_id) VALUES (?1, ?2)",
                (&book.id, author_id),
            )
            .unwrap();

        return self;
    }

    pub fn with_borrow_entry(self, entry_id: &Uuid, user: &User, book: &Book, timestamp: DateTime<Utc>) -> MockDatabaseBuilder {
        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO map_users_to_borrowed_books (id, user_id, book_id, timestamp, action) VALUES (?1, ?2, ?3, ?4, ?5)",
                (entry_id, &user.id, &book.id, timestamp, BookBorrowState::Borrowed),
            )
            .unwrap();

        return self;
    }

    pub fn with_return_entry(self, entry_id: &Uuid, user: &User, book: &Book, timestamp: DateTime<Utc>) -> MockDatabaseBuilder {
        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO map_users_to_borrowed_books (id, user_id, book_id, timestamp, action) VALUES (?1, ?2, ?3, ?4, ?5)",
                (entry_id, &user.id, &book.id, timestamp, BookBorrowState::Returned),
            )
            .unwrap();

        return self;
    }

    pub fn build(self) -> Pool<SqliteConnectionManager> {
        return self.connection;
    }
}

pub struct MockDatabaseQuerier {
    pool: Pool<SqliteConnectionManager>,
}

impl MockDatabaseQuerier {
    pub fn create(database_path: String) -> MockDatabaseQuerier {
        let manager = SqliteConnectionManager::file(database_path);
        return MockDatabaseQuerier {
            pool: r2d2::Pool::new(manager).unwrap(),
        };
    }

    pub fn contains_num_books(&self, num: i32) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM books",
            (),
            |row| Ok(row.get(0)?),
        ) {
            Ok(count) => return count == num,
            Err(_) => return false,
        }
    }

    pub fn contains_num_authors(&self, num: i32) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM authors",
            (),
            |row| Ok(row.get(0)?),
        ) {
            Ok(count) => return count == num,
            Err(_) => return false,
        }
    }

    pub fn contains_book(&self, book: &Book) -> bool {
        match self.pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM books WHERE id = ?1 AND name = ?2 AND description = ?3 AND language = ?4", 
            (&book.id, &book.name, &book.description, &book.language),
            |row| Ok(row.get(0)?)
        ) {
            Ok(count) => return count == 1,
            Err(_) => return false,
        }
    }

    pub fn contains_author(&self, author: &Author) -> bool {
        match self.pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM authors WHERE id = ?1 AND name = ?2 AND description = ?3 AND country = ?4", 
            (&author.id, &author.name, &author.description, &author.country),
            |row| Ok(row.get(0)?)
        ) {
            Ok(count) => return count == 1,
            Err(_) => return false,
        }
    }

    pub fn contains_book_author_mapping(&self, book_id: &Uuid, author_id: &Uuid) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM map_books_to_authors WHERE book_id = ?1 AND author_id = ?2",
            (book_id, author_id),
            |row| Ok(row.get(0)?),
        ) {
            Ok(count) => return count == 1,
            Err(_) => return false,
        }
    }
}

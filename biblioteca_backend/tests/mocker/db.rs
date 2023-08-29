use std::fs::remove_file;

use biblioteca_backend::{catalog::model::{Author, Book}, database::setup_db};
use r2d2::{Pool, PooledConnection};
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
        self.connection.get().unwrap().execute(
            "INSERT INTO authors (id, name, description, country) VALUES (?1, ?2, ?3, ?4)",
            (&author.id, &author.name, &author.description, &author.country),
        ).unwrap();

        return self;
    }

    pub fn with_book(self, book: &Book, author_id: &Uuid) -> MockDatabaseBuilder {
        self.connection.get().unwrap().execute(
            "INSERT INTO books (id, name, description, language) VALUES (?1, ?2, ?3, ?4)",
            (&book.id, &book.name, &book.description, &book.language),
        ).unwrap();

        self.connection.get().unwrap().execute(
            "INSERT INTO map_books_to_authors (book_id, author_id) VALUES (?1, ?2)",
            (&book.id, author_id),
        ).unwrap();

        return self;
    }

    pub fn build(self) -> Pool<SqliteConnectionManager> {
        return self.connection;
    }
}

pub struct MockDatabaseQuerier {}

impl MockDatabaseQuerier {
    pub fn new_pool(database_path: String) -> Pool<SqliteConnectionManager> {
        let manager = SqliteConnectionManager::file(database_path);
        return r2d2::Pool::new(manager).unwrap();
    }

    pub fn contains_num_books(pool: &Pool<SqliteConnectionManager>, num: i32) -> bool {
        match pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM books", 
            (),
            |row| Ok(row.get(0)?)
        ) {
            Ok(count) => return count == num,
            Err(_) => return false,
        }
    }

    pub fn contains_book(pool: &Pool<SqliteConnectionManager>, book: &Book) -> bool {
        match pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM books WHERE id = ?1 AND name = ?2 AND description = ?3 AND language = ?4", 
            (&book.id, &book.name, &book.description, &book.language),
            |row| Ok(row.get(0)?)
        ) {
            Ok(count) => return count == 1,
            Err(_) => return false,
        }
    }

    pub fn contains_author(pool: &Pool<SqliteConnectionManager>, author: &Author) -> bool {
        match pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM authors WHERE id = ?1 AND name = ?2 AND description = ?3 AND country = ?4", 
            (&author.id, &author.name, &author.description, &author.country),
            |row| Ok(row.get(0)?)
        ) {
            Ok(count) => return count == 1,
            Err(_) => return false,
        }
    }

    pub fn contains_book_author_mapping(pool: &Pool<SqliteConnectionManager>, book_id: &Uuid, author_id: &Uuid) -> bool {
        match pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM map_books_to_authors WHERE book_id = ?1 AND author_id = ?2", 
            (book_id, author_id),
            |row| Ok(row.get(0)?)
        ) {
            Ok(count) => return count == 1,
            Err(_) => return false,
        }
    }
}
use std::fs::remove_file;

use biblioteca_backend::{
    catalog::model::{Author, Book},
    database::setup_db,
    library::model::{BookBorrowEntry, BookBorrowState},
    users::model::{User, UserRole},
};

use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use uuid::Uuid;

pub struct MockDatabaseBuilder {
    pub connection: Pool<SqliteConnectionManager>,
}

impl MockDatabaseBuilder {
    pub fn create(database_path: String) -> MockDatabaseBuilder {
        MockDatabaseBuilder {
            connection: setup_db(database_path).unwrap(),
        }
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

        self
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

        self
    }

    pub fn with_user(self, user: &User, user_role: &UserRole) -> MockDatabaseBuilder {
        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO users (id, username) VALUES (?1, ?2)",
                (&user.id, &user.username),
            )
            .unwrap();

        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO map_users_to_user_roles (user_id, user_role_id) VALUES (?1, ?2)",
                (&user.id, &user_role.id),
            )
            .unwrap();

        self
    }

    pub fn with_user_role(self, user_role: &UserRole) -> MockDatabaseBuilder {
        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO user_roles (id, name, num_borrowable_books) VALUES (?1, ?2, ?3)",
                (
                    &user_role.id,
                    &user_role.name,
                    &user_role.num_borrowable_books,
                ),
            )
            .unwrap();

        self
    }

    pub fn with_library_entry(self, book_borrow_entry: &BookBorrowEntry) -> MockDatabaseBuilder {
        self.connection
            .get()
            .unwrap()
            .execute(
                "INSERT INTO map_users_to_borrowed_books (id, user_id, book_id, timestamp, action) VALUES (?1, ?2, ?3, ?4, ?5)",
                (book_borrow_entry.id, &book_borrow_entry.user_id, &book_borrow_entry.book_id, book_borrow_entry.timestamp, book_borrow_entry.state),
            )
            .unwrap();

        self
    }

    pub fn build(self) -> Pool<SqliteConnectionManager> {
        self.connection
    }
}

pub struct MockDatabaseQuerier {
    pool: Pool<SqliteConnectionManager>,
}

impl MockDatabaseQuerier {
    pub fn create(database_path: String) -> MockDatabaseQuerier {
        let manager = SqliteConnectionManager::file(database_path);
        MockDatabaseQuerier {
            pool: r2d2::Pool::new(manager).unwrap(),
        }
    }

    pub fn contains_num_books(&self, num: i32) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM books",
            (),
            |row| row.get(0),
        ) {
            Ok(count) => count == num,
            Err(_) => false,
        }
    }

    pub fn contains_num_authors(&self, num: i32) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM authors",
            (),
            |row| row.get(0),
        ) {
            Ok(count) => count == num,
            Err(_) => false,
        }
    }

    pub fn contains_num_users(&self, num: i32) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM users",
            (),
            |row| row.get(0),
        ) {
            Ok(count) => count == num,
            Err(_) => false,
        }
    }

    pub fn contains_num_user_roles(&self, num: i32) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM user_roles",
            (),
            |row| row.get(0),
        ) {
            Ok(count) => count == num,
            Err(_) => false,
        }
    }

    pub fn contains_book(&self, book: &Book) -> bool {
        match self.pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM books WHERE id = ?1 AND name = ?2 AND description = ?3 AND language = ?4", 
            (&book.id, &book.name, &book.description, &book.language),
            |row| row.get(0)
        ) {
            Ok(count) => count == 1,
            Err(_) => false,
        }
    }

    pub fn contains_author(&self, author: &Author) -> bool {
        match self.pool.get().unwrap().query_row::<i32,_,_>(
            "SELECT COUNT(*) FROM authors WHERE id = ?1 AND name = ?2 AND description = ?3 AND country = ?4", 
            (&author.id, &author.name, &author.description, &author.country),
            |row| row.get(0)
        ) {
            Ok(count) => count == 1,
            Err(_) => false,
        }
    }

    pub fn contains_book_author_mapping(&self, book_id: &Uuid, author_id: &Uuid) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM map_books_to_authors WHERE book_id = ?1 AND author_id = ?2",
            (book_id, author_id),
            |row| row.get(0),
        ) {
            Ok(count) => count == 1,
            Err(_) => false,
        }
    }

    pub fn contains_user(&self, user: &User) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM users WHERE id = ?1 AND username = ?2",
            (&user.id, &user.username),
            |row| row.get(0),
        ) {
            Ok(count) => count == 1,
            Err(_) => false,
        }
    }

    pub fn contains_user_role(&self, user_role: &UserRole) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM user_roles WHERE id = ?1 AND name = ?2 AND num_borrowable_books = ?3",
            (&user_role.id, &user_role.name, &user_role.num_borrowable_books),
            |row| row.get(0)
        ) {
            Ok(count) => count == 1,
            Err(_) => false,
        }
    }

    pub fn contains_user_user_role_mapping(&self, user_id: &Uuid, user_role_id: &Uuid) -> bool {
        match self.pool.get().unwrap().query_row::<i32, _, _>(
            "SELECT COUNT(*) FROM map_users_to_user_roles WHERE user_id = ?1 AND user_role_id = ?2",
            (&user_id, &user_role_id),
            |row| row.get(0),
        ) {
            Ok(count) => count == 1,
            Err(_) => false,
        }
    }

    pub fn is_book_borrowed(&self, book_id: &Uuid) -> bool {
        match self.pool.get().unwrap().query_row(
            "SELECT * FROM map_users_to_borrowed_books WHERE book_id = ?1 ORDER BY timestamp DESC",
            [book_id],
            |row| {
                Ok(BookBorrowEntry {
                    id: row.get(0)?,
                    user_id: row.get(1)?,
                    book_id: row.get(2)?,
                    timestamp: row.get(3)?,
                    state: row.get(4)?,
                })
            },
        ) {
            Ok(entry) => match entry.state {
                BookBorrowState::Borrowed => true,
                BookBorrowState::Returned => false,
            },
            Err(_) => false,
        }
    }

    pub fn is_book_returned(&self, book_id: &Uuid) -> bool {
        !self.is_book_borrowed(book_id)
    }
}

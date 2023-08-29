use biblioteca_backend::{catalog::model::{Author, Book}, database::setup_db};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use uuid::Uuid;

pub struct MockDatabaseBuilder {
    connection: Pool<SqliteConnectionManager>,
}

impl MockDatabaseBuilder {
    pub fn create(database_path: String) -> MockDatabaseBuilder {
        return MockDatabaseBuilder { 
            connection: setup_db(database_path).unwrap(),
        };
    }

    pub fn with_author(&self, author: Author) -> &MockDatabaseBuilder {
        self.connection.get().unwrap().execute(
            "INSERT INTO authors (id, name, description, country) VALUES (?1, ?2, ?3, ?4)",
            (&author.id, &author.name, &author.description, &author.country),
        ).unwrap();

        return self;
    }

    pub fn with_book(&self, book: Book, author_id: Uuid) -> &MockDatabaseBuilder {
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
}
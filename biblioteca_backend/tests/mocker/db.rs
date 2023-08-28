use biblioteca_backend::catalog::model::{Author, Book};
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use uuid::Uuid;

pub struct MockDatabaseBuilder {
    connection: PooledConnection<SqliteConnectionManager>,
}

impl MockDatabaseBuilder {
    pub fn with_author(&self, author: Author) -> &MockDatabaseBuilder {
        self.connection.execute(
            "INSERT INTO authors (id, name, description, country) VALUES (?1, ?2, ?3, ?4)",
            (&author.id, &author.name, &author.description, &author.country),
        ).unwrap();

        return self;
    }

    pub fn with_book(&self, book: Book, author_id: Uuid) -> &MockDatabaseBuilder {
        self.connection.execute(
            "INSERT INTO books (id, name, description, language) VALUES (?1, ?2, ?3, ?4)",
            (&book.id, &book.name, &book.description, &book.language),
        ).unwrap();

        self.connection.execute(
            "INSERT INTO map_books_to_authors (book_id, author_id) VALUES (?1, ?2)",
            (&book.id, author_id),
        ).unwrap();

        return self;
    }
}
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result};
use uuid::Uuid;

use crate::catalog::model::Book;

pub fn setup_db() -> Result<Pool<SqliteConnectionManager>> {
    tracing::debug!("Setting up our in-memory, SQLite database...");

    let manager = SqliteConnectionManager::file("library.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    tracing::debug!("Creating table 'books'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS books (
                id              BLOB PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL
            )", 
            (),
        )?;

    tracing::debug!("Creating table 'authors'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS authors (
                id              BLOB PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT,
                country         TEXT NOT NULL,
                language        TEXT NOT NULL
            )",
            ()
        )?;

    tracing::debug!("Creating table 'book_authors'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS book_authors (
                book_id     BLOB PRIMARY KEY,
                author_id   BLOB NOT NULL,
                CONSTRAINT fk_books
                    FOREIGN KEY(book_id) REFERENCES books(id)
                    ON DELETE CASCADE
                CONSTRAINT fk_authors
                    FOREIGN KEY(author_id) REFERENCES authors(id)
                    ON DELETE CASCADE
            )", 
            ()
        )?;

    tracing::debug!("Database setup complete! :)");
    Ok(pool)
}

pub fn insert_mock_data() -> Result<()> {
    let conn = Connection::open("library.db")?;

    let book = Book {
        id: Uuid::new_v4(),
        name: "Harry Potter and the Philosopher's Stone".to_string(),
        description: "The boy who lived starts his journey.".to_string(),
    };

    conn.execute(
        "INSERT INTO books (id, name, description) VALUES (?1, ?2, ?3)",
        (&book.id, &book.name, &book.description),
    )?;

    Ok(())
}
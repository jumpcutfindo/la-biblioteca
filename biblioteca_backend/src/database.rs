use rusqlite::{Connection, Result};
use uuid::Uuid;

use crate::catalog::model::Book;

pub fn setup_db() -> Result<()> {
    tracing::debug!("Setting up our in-memory, SQLite database...");
    let conn = Connection::open("library.db")?;

    tracing::debug!("Creating table 'books'...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS books (
            id              BLOB PRIMARY KEY,
            name            TEXT NOT NULL,
            description     TEXT NOT NULL
        )", 
        (),
    )?;

    tracing::debug!("Creating table 'authors'...");
    conn.execute(
        "CREATE TABLE IF NOT EXISTS authors (
            id              BLOB PRIMARY KEY,
            name            TEXT NOT NULL,
            description     TEXT,
            country         TEXT NOT NULL,
            language        TEXT NOT NULL
        )",
        ()
    )?;
    
    tracing::debug!("Database setup complete! :)");
    Ok(())
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
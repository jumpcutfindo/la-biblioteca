use rusqlite::{Connection, Result};

use crate::catalog_model::Book;

pub async fn setup_db() -> Result<()> {
    let conn = Connection::open_in_memory()?;

    let _ = conn.execute(
        "CREATE TABLE books (
            id              INTEGER PRIMARY KEY
            name            TEXT NOT NULL
            description     TEXT NOT NULL
        )", 
        (),
    );

    let book = Book {
        id: 0,
        name: "Harry Potter and the Philosopher's Stone".to_string(),
        description: "The boy who lived starts his journey.".to_string(),
    };

    conn.execute(
        "INSERT INTO books (id, name, description) VALUES (?1, ?2, ?3)",
        (&book.id, &book.name, &book.description),
    )?;

    Ok(())
}
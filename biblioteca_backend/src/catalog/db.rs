use std::collections::HashMap;

use axum::extract::State;
use rusqlite::Result;
use uuid::Uuid;

use crate::app::AppState;

use super::model::{Author, Book};

pub async fn list_books_from_db(
    State(state): State<AppState>,
    params: HashMap<String, String>,
) -> Result<Vec<Book>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt_string = String::from("SELECT * FROM books WHERE 1=1");

    if params.contains_key("name") {
        stmt_string.push_str(&format!(
            " AND name LIKE '%{}%'",
            params.get("name").unwrap()
        ));
    }

    if params.contains_key("language") {
        stmt_string.push_str(&format!(
            " AND language LIKE '%{}%'",
            params.get("language").unwrap()
        ));
    }

    let mut stmt = conn.prepare(&stmt_string)?;

    let books = stmt
        .query_map([], |row| {
            Ok(Book {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                language: row.get(3)?,
            })
        })?
        .map(|book| book.unwrap())
        .collect();

    Ok(books)
}

pub async fn get_book_from_db(State(state): State<AppState>, id: Uuid) -> Result<Book> {
    state
        .db_pool
        .get()
        .unwrap()
        .query_row("SELECT * FROM books WHERE id = $1", [id], |row| {
            Ok(Book {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                language: row.get(3)?,
            })
        })
}

pub async fn add_book_to_db(
    State(state): State<AppState>,
    book: Book,
    author_id: Uuid,
) -> Result<Book, rusqlite::Error> {
    let mut conn = state.db_pool.get().unwrap();

    // Use transaction to ensure both statements complete
    let tx = conn.transaction()?;

    // Add the book itself
    tx.execute(
        "INSERT INTO books (id, name, description, language) VALUES (?1, ?2, ?3, ?4)",
        (&book.id, &book.name, &book.description, &book.language),
    )?;

    // Add link between author and book
    match tx.execute(
        "INSERT INTO map_books_to_authors (book_id, author_id) VALUES (?1, ?2)",
        (&book.id, author_id),
    ) {
        Ok(_it) => {}
        Err(err) => return Err(err),
    };

    tx.commit()?;

    Ok(book)
}

pub async fn delete_book_from_db(State(state): State<AppState>, id: Uuid) -> Result<()> {
    state
        .db_pool
        .get()
        .unwrap()
        .execute("DELETE FROM books WHERE id = $1", [id])?;

    Ok(())
}

pub async fn update_book_in_db(
    State(state): State<AppState>,
    book: Book,
    author_id: Uuid,
) -> Result<(), rusqlite::Error> {
    let mut conn = state.db_pool.get().unwrap();
    let tx = conn.transaction()?;

    // Update entry
    tx.execute(
        "UPDATE books
        SET name = $1,
            description = $2,
            language = $3
        WHERE
            id = $4;
        ",
        (book.name, book.description, book.language, book.id),
    )?;

    // Update association
    match tx.execute(
        "UPDATE map_books_to_authors
        SET author_id = $1
        WHERE book_id = $2",
        (author_id, book.id),
    ) {
        Ok(_) => {}
        Err(err) => return Err(err),
    };

    tx.commit()?;

    Ok(())
}

pub async fn list_authors_from_db(
    State(state): State<AppState>,
    params: HashMap<String, String>,
) -> Result<Vec<Author>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt_string = String::from("SELECT * FROM authors WHERE 1=1");

    if params.contains_key("name") {
        stmt_string.push_str(&format!(" AND name LIKE '%{}%'", params.get("name").unwrap()));
    }

    if params.contains_key("country") {
        stmt_string.push_str(&format!(
            " AND country LIKE '%{}%'",
            params.get("country").unwrap()
        ));
    }

    let mut stmt = conn.prepare(&stmt_string)?;

    let authors = stmt
        .query_map([], |row| {
            Ok(Author {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                country: row.get(3)?,
            })
        })?
        .map(|author| author.unwrap())
        .collect();

    Ok(authors)
}

pub async fn get_author_from_db(State(state): State<AppState>, id: Uuid) -> Result<Author> {
    state
        .db_pool
        .get()
        .unwrap()
        .query_row("SELECT * FROM authors WHERE id = $1", [id], |row| {
            Ok(Author {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                country: row.get(3)?,
            })
        })
}

pub async fn add_author_to_db(State(state): State<AppState>, author: Author) -> Result<Author> {
    state.db_pool.get().unwrap().execute(
        "INSERT INTO authors (id, name, description, country) VALUES (?1, ?2, ?3, ?4)",
        (
            &author.id,
            &author.name,
            &author.description,
            &author.country,
        ),
    )?;

    Ok(author)
}

pub async fn delete_author_from_db(State(state): State<AppState>, id: Uuid) -> Result<()> {
    state
        .db_pool
        .get()
        .unwrap()
        .execute("DELETE FROM authors WHERE id = $1", [id])?;

    Ok(())
}

pub async fn update_author_in_db(State(state): State<AppState>, author: Author) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "UPDATE authors
        SET name = $1,
            description = $2,
            country = $3
        WHERE
            id = $5;
        ",
        (author.name, author.description, author.country, author.id),
    )?;

    Ok(())
}

pub fn is_author_exists_in_db(
    State(state): &State<AppState>,
    author_id: Uuid,
) -> Result<bool, rusqlite::Error> {
    match state.db_pool.get().unwrap().query_row::<i32, _, _>(
        "SELECT COUNT(*) FROM authors WHERE id = $1",
        [author_id],
        |row| Ok(row.get(0)?),
    ) {
        Ok(count) => return Ok(count > 0),
        Err(err) => return Err(err),
    }
}

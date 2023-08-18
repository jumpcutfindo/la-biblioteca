use std::collections::HashMap;

use axum::extract::State;
use rusqlite::{ Connection, Result, Statement };
use uuid::Uuid;

use crate::AppState;

use super::{model::{Book, Author}, error::CatalogError};

pub async fn list_books_from_db(
    State(state): State<AppState>,
    params: HashMap<String, String>,
) -> Result<Vec<Book>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt_string = String::from("SELECT * FROM books WHERE 1=1");

    if params.contains_key("name") {
        stmt_string.push_str(&format!(" AND name LIKE '%{}%'", params.get("name").unwrap()));
    }

    let mut stmt = conn.prepare(&stmt_string)?;

    let books = stmt
    .query_map([], |row| {
        Ok(Book {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
        })
    })?
    .map(|book| book.unwrap())
    .collect();

    Ok(books)
}

pub async fn get_book_from_db(
    State(state): State<AppState>, 
    id: Uuid,
) -> Result<Book> {
    state.db_pool.get().unwrap().query_row(
        "SELECT * FROM books WHERE id = $1", 
        [id], 
        |row| {
            Ok(Book {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
            })
        })
}

pub async fn add_book_to_db(
    State(state): State<AppState>,
    book: Book,
    author_id: Uuid,
) -> Result<Book, CatalogError> {
    let mut conn = state.db_pool.get().unwrap();

    // Use transaction to ensure both statements complete
    let tx = conn.transaction()?;

    // Add the book itself
    tx.execute(
        "INSERT INTO books (id, name, description) VALUES (?1, ?2, ?3)",
        (&book.id, &book.name, &book.description),
    )?;

    // Add link between author and book
    match tx.execute(
        "INSERT INTO map_books_to_authors (book_id, author_id) VALUES (?1, ?2)",
        (&book.id, author_id),
    ) {
        Ok(_it) => {},
        Err(err) => {
            match err.sqlite_error_code().unwrap() {
                rusqlite::ErrorCode::ConstraintViolation => 
                    return Err(CatalogError::AuthorNotFound),
                _ => return Err(CatalogError::DatabaseError(err))
            }
        }
    };

    tx.commit()?;

    Ok(book)
}

pub async fn delete_book_from_db(
    State(state): State<AppState>,
    id: Uuid
) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "DELETE FROM books WHERE id = $1",
        [id],
    )?;

    Ok(())
}

pub async fn update_book_in_db(
    State(state): State<AppState>,
    book: Book,
    author_id: Uuid,
) -> Result<(), CatalogError> {
    let mut conn = state.db_pool.get().unwrap();
    let tx = conn.transaction()?;

    // Update entry
    tx.execute(
        "UPDATE books
        SET name = $1,
            description = $2
        WHERE
            id = $3;
        ",
        (book.name, book.description, book.id),
    )?;

    // Update association
    match tx.execute(
        "UPDATE map_books_to_authors
        SET author_id = $1
        WHERE book_id = $2",
        (author_id, book.id) 
    ) {
        Ok(_it) => {},
        Err(err) => {
            match err.sqlite_error_code().unwrap() {
                rusqlite::ErrorCode::ConstraintViolation => 
                    return Err(CatalogError::AuthorNotFound),
                _ => return Err(CatalogError::DatabaseError(err))
            }
        }
    };

    tx.commit()?;

    Ok(())
}

pub async fn list_authors_from_db(
    State(state): State<AppState>,
) -> Result<Vec<Author>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM authors")?;

    let authors = stmt
    .query_map([], |row| {
        Ok(Author {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            country: row.get(3)?,
            language: row.get(4)?,
        })
    })?
    .map(|author| author.unwrap())
    .collect();

    Ok(authors)
}

pub async fn get_author_from_db(
    State(state): State<AppState>, 
    id: Uuid,
) -> Result<Author> {
    state.db_pool.get().unwrap().query_row(
        "SELECT * FROM authors WHERE id = $1",
        [id],
        |row| {
            Ok(Author {
                id: row.get(0)?,
                name: row.get(1)?,
                description: row.get(2)?,
                country: row.get(3)?,
                language: row.get(4)?,
            })
        })
}

pub async fn add_author_to_db(
    State(state): State<AppState>,
    author: Author,
) -> Result<Author> {
    state.db_pool.get().unwrap().execute(
        "INSERT INTO authors (id, name, description, country, language) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&author.id, &author.name, &author.description, &author.country, &author.language),
    )?;

    Ok(author)
}

pub async fn delete_author_from_db(
    State(state): State<AppState>,
    id: Uuid
) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "DELETE FROM authors WHERE id = $1",
        [id],
    )?;

    Ok(())
}

pub async fn update_author_in_db(
    State(state): State<AppState>,
    author: Author
) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "UPDATE authors
        SET name = $1,
            description = $2,
            country = $3,
            language = $4
        WHERE
            id = $5;
        ",
        (author.name, author.description, author.country, author.language, author.id),
    )?;

    Ok(())
}

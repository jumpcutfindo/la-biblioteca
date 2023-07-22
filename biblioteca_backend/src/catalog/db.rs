use axum::extract::State;
use rusqlite::{ Connection, Result, Statement };
use uuid::Uuid;

use crate::AppState;

use super::{model::{Book, Author}, error::CatalogError};

pub async fn get_all_books_from_db(
    State(state): State<AppState>,
) -> Result<Vec<Book>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM books")?;

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
    let conn = state.db_pool.get().unwrap();
    
    let author_exists = conn.query_row(
        "SELECT EXISTS (SELECT 1 FROM authors WHERE id = ?1)", 
        [author_id],
        |row| {
            let val: i32 = row.get(0)?;
            Ok(val)
        }
    );

    if author_exists.unwrap() == 0 {
        return Err(CatalogError::AuthorNotFound)
    }

    // Add the book itself
    match conn.execute(
            "INSERT INTO books (id, name, description) VALUES (?1, ?2, ?3)",
            (&book.id, &book.name, &book.description),
        ) {
        Ok(_it) => return Ok(book),
        Err(err) => {
            tracing::warn!("{}", err);
            return Err(CatalogError::DatabaseError(err))
        },
    };
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
) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "UPDATE books
        SET name = $1,
            description = $2
        WHERE
            id = $3;
        ",
        (book.name, book.description, book.id),
    )?;

    Ok(())
}

pub async fn get_all_authors_from_db(
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

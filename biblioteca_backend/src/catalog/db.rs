use axum::extract::State;
use rusqlite::{ Connection, Result };
use uuid::Uuid;

use crate::AppState;

use super::model::{Book, Author};

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
) -> Result<Book> {
    state.db_pool.get().unwrap().execute(
        "INSERT INTO books (id, name, description) VALUES (?1, ?2, ?3)",
        (&book.id, &book.name, &book.description),
    )?;

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

pub async fn get_all_authors_from_db() {
}

pub async fn get_author_from_db(id: Uuid) {
}

pub async fn add_author_to_db(author: Author) {
}

pub async fn delete_author_from_db(id: Uuid) {

}

pub async fn update_author_in_db(author: Author) {
    
}

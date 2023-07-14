use rusqlite:: { Connection, Result };

use super::model::Book;

pub async fn add_book_to_db(book: Book) -> Result<Book> {
    let conn = Connection::open("library.db")?;

    conn.execute(
        "INSERT INTO books (id, name, description) VALUES (?1, ?2, ?3)",
        (&book.id, &book.name, &book.description),
    )?;

    Ok(book)
}

pub async fn get_all_books_from_db() -> Result<Vec<Book>> {
    let conn = Connection::open("library.db")?;

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
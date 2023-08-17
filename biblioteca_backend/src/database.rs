use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{Connection, Result};
use uuid::Uuid;

use crate::catalog::model::Book;

pub fn setup_db() -> Result<Pool<SqliteConnectionManager>> {
    tracing::debug!("Setting up our in-memory, SQLite database...");

    let manager = SqliteConnectionManager::file("library.db");
    let pool = r2d2::Pool::new(manager).unwrap();

    setup_catalog_tables(&pool);
    setup_user_tables(&pool);
    setup_library_tables(&pool);
        
    tracing::debug!("Database setup complete! :)");
    Ok(pool)
}

fn setup_catalog_tables(pool: &Pool<SqliteConnectionManager>) {
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
        )
        .unwrap();

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
        )
        .unwrap();

    tracing::debug!("Creating table 'map_books_to_authors'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS map_books_to_authors (
                book_id     BLOB PRIMARY KEY,
                author_id   BLOB NOT NULL,
                CONSTRAINT fk_books
                    FOREIGN KEY(book_id) REFERENCES books(id)
                    ON DELETE CASCADE,
                CONSTRAINT fk_authors
                    FOREIGN KEY(author_id) REFERENCES authors(id)
                    ON DELETE CASCADE
            )", 
            ()
        )
        .unwrap();
}

fn setup_user_tables(pool: &Pool<SqliteConnectionManager>) {
    tracing::debug!("Creating table 'user_roles'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS user_roles (
                id                      BLOB PRIMARY KEY,
                role_name               TEXT NOT NULL,
                num_borrowable_books    INT NOT NULL
            )",
            ()
        )
        .unwrap();

    tracing::debug!("Inserting some default roles into 'user_roles'...");
    let binding = pool.get().unwrap();
    let mut user_role_stmt = binding.prepare(
        "INSERT OR IGNORE INTO user_roles (id, role_name, num_borrowable_books) VALUES (?1, ?2, ?3)"
    )
    .unwrap();

    user_role_stmt.execute((Uuid::parse_str("f4658962-1237-4518-b55c-1f44986a4604").unwrap(), String::from("admin"), 0)).unwrap();
    user_role_stmt.execute((Uuid::parse_str("ded1bba9-84aa-4138-8f71-b27cfe6a51a0").unwrap(), String::from("adult_user"), 8)).unwrap();
    user_role_stmt.execute((Uuid::parse_str("27b122ab-b9e7-4f9b-ad7e-368340cfec76").unwrap(), String::from("child_user"), 4)).unwrap();

    tracing::debug!("Creating table 'users'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id              BLOB PRIMARY KEY,
                username        TEXT UNIQUE NOT NULL
            )", 
            ()
        )
        .unwrap();

    tracing::debug!("Creating table 'map_users_to_user_roles'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS map_users_to_user_roles (
                user_id             BLOB NOT NULL,
                user_role_id        BLOB NOT NULL,
                CONSTRAINT fk_users
                    FOREIGN KEY(user_id) REFERENCES users(id)
                    ON DELETE CASCADE,
                CONSTRAINT fk_user_roles
                    FOREIGN KEY(user_role_id) REFERENCES user_roles(id)
            )",
        ()
        )
        .unwrap();
}

fn setup_library_tables(pool: &Pool<SqliteConnectionManager>) {
    tracing::debug!("Creating table 'map_users_to_borrowed_books'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS map_users_to_borrowed_books (
                user_id     BLOB NOT NULL,
                book_id     BLOB NOT NULL,
                CONSTRAINT fk_users
                    FOREIGN KEY (user_id) REFERENCES users(id)
                    ON DELETE CASCADE,
                CONSTRAINT fk_books
                    FOREIGN KEY (book_id) REFERENCES books(id)
                    ON DELETE CASCADE
            )", 
            ()
        )
        .unwrap();
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
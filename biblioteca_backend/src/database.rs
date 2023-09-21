use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Result;
use uuid::Uuid;

pub fn setup_db(database_path: String) -> Result<Pool<SqliteConnectionManager>> {
    tracing::debug!("Setting up our in-memory, SQLite database...");

    let manager = SqliteConnectionManager::file(database_path);
    let pool = r2d2::Pool::new(manager).unwrap();

    setup_catalog_tables(&pool);
    setup_user_tables(&pool);
    setup_library_tables(&pool);

    tracing::debug!("Database setup complete! :)");
    Ok(pool)
}

fn setup_catalog_tables(pool: &Pool<SqliteConnectionManager>) {
    tracing::debug!("Creating 'catalog' related tables...");
    tracing::debug!("> Creating table 'books'...");

    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS books (
                id              BLOB PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT NOT NULL,
                language        TEXT NOT NULL
            )",
            (),
        )
        .unwrap();

    tracing::debug!("> Creating table 'authors'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS authors (
                id              BLOB PRIMARY KEY,
                name            TEXT NOT NULL,
                description     TEXT,
                country         TEXT NOT NULL
            )",
            (),
        )
        .unwrap();

    tracing::debug!("> Creating table 'map_books_to_authors'...");
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
            (),
        )
        .unwrap();
}

fn setup_user_tables(pool: &Pool<SqliteConnectionManager>) {
    tracing::debug!("Creating 'user' related tables...");
    tracing::debug!("> Creating table 'user_roles'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS user_roles (
                id                      BLOB PRIMARY KEY,
                role_name               TEXT NOT NULL,
                num_borrowable_books    INT NOT NULL
            )",
            (),
        )
        .unwrap();

    tracing::debug!("> Inserting some default roles into 'user_roles'...");
    let binding = pool.get().unwrap();
    let mut user_role_stmt = binding.prepare(
        "INSERT OR IGNORE INTO user_roles (id, role_name, num_borrowable_books) VALUES (?1, ?2, ?3)"
    )
    .unwrap();

    tracing::debug!("> Creating table 'users'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS users (
                id              BLOB PRIMARY KEY,
                username        TEXT UNIQUE NOT NULL
            )",
            (),
        )
        .unwrap();

    tracing::debug!("> Creating table 'map_users_to_user_roles'...");
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
            (),
        )
        .unwrap();
}

fn setup_library_tables(pool: &Pool<SqliteConnectionManager>) {
    tracing::debug!("Creating 'library' related tables...");
    tracing::debug!("> Creating table 'map_users_to_borrowed_books'...");
    pool.get()
        .unwrap()
        .execute(
            "CREATE TABLE IF NOT EXISTS map_users_to_borrowed_books (
                id              BLOB NOT NULL,
                user_id         BLOB NOT NULL,
                book_id         BLOB NOT NULL,
                timestamp       DATE NOT NULL,
                action          TEXT NOT NULL,
                CONSTRAINT fk_users
                    FOREIGN KEY (user_id) REFERENCES users(id)
                    ON DELETE CASCADE,
                CONSTRAINT fk_books
                    FOREIGN KEY (book_id) REFERENCES books(id)
                    ON DELETE CASCADE
            )",
            (),
        )
        .unwrap();
}

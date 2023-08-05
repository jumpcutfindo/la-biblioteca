use axum::{extract::State};
use rusqlite::{Result, Error};
use uuid::Uuid;

use crate::AppState;

use super::model::{User, UserRole};

pub async fn list_users_from_db(
    State(state): State<AppState>,
) -> Result<Vec<User>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM users")?;

    let users = stmt
        .query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                username: row.get(1)?,
            })
        })?
        .map(|user| user.unwrap())
        .collect();

    Ok(users)
}

pub async fn list_user_roles_from_db(
    State(state): State<AppState>,
) -> Result<Vec<UserRole>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt = conn.prepare("SELECT * FROM user_roles")?;

    let user_roles = stmt
        .query_map([], |row| {
            Ok(UserRole {
                id: row.get(0)?,
                name: row.get(1)?,
                num_borrowable_books: row.get(2)?,
            })
        })?
        .map(|user| user.unwrap())
        .collect();

    Ok(user_roles)
}

pub async fn get_user_from_db(
    State(state): State<AppState>,
    id: Uuid
) -> Result<User> {
    state.db_pool.get().unwrap().query_row(
        "SELECT * FROM users WHERE id = $1", 
        [id], 
    |row| {
        Ok(User {
            id: row.get(0)?,
            username: row.get(1)?,
        })
    })
}

pub async fn get_all_user_roles_from_db() {

}

pub async fn add_user_to_db(
    State(state): State<AppState>,
    user: User,
    user_role_id: Uuid,
) -> Result<User, Error>{
    let mut conn = state.db_pool.get().unwrap();

    let tx = conn.transaction()?;

    // Add the user itself
    tx.execute(
        "INSERT INTO users (id, username) VALUES (?1, ?2)",
        (&user.id, &user.username),
    )?;

    // Add the user's role association
    tx.execute(
        "INSERT INTO map_users_to_user_roles (user_id, user_role_id) VALUES (?1, ?2)",
        (&user.id, user_role_id),
    )?;

    tx.commit()?;

    Ok(user)
}

pub async fn delete_user_from_db(
    State(state): State<AppState>,
    id: Uuid,
) -> Result<()> {
    state.db_pool.get().unwrap().execute(
        "DELETE FROM users WHERE id = $1", 
        [id],
    )?;

    Ok(())
}
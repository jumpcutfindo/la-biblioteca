use axum::extract::State;
use rusqlite::{Error, Result};
use uuid::Uuid;

use crate::app::AppState;

use super::model::{FullUser, User, UserRole};

pub async fn list_users_from_db(State(state): State<AppState>) -> Result<Vec<FullUser>> {
    let conn = state.db_pool.get().unwrap();

    let mut stmt = conn.prepare("
        SELECT a.id as user_id, a.username, c.id as user_role_id, c.role_name, c.num_borrowable_books 
        FROM users a, map_users_to_user_roles b, user_roles c 
        WHERE a.id = b.user_id AND b.user_role_id = c.id"
    )?;

    let users = stmt
        .query_map([], |row| {
            Ok(FullUser {
                id: row.get(0)?,
                username: row.get(1)?,
                user_role: UserRole {
                    id: row.get(2)?,
                    name: row.get(3)?,
                    num_borrowable_books: row.get(4)?,
                },
            })
        })?
        .map(|user| user.unwrap())
        .collect();

    Ok(users)
}

pub async fn get_user_from_db(State(state): State<AppState>, id: Uuid) -> Result<FullUser> {
    state.db_pool.get().unwrap().query_row(
        "SELECT a.id as user_id, a.username, c.id as user_role_id, c.role_name, c.num_borrowable_books 
        FROM users a, map_users_to_user_roles b, user_roles c 
        WHERE a.id = b.user_id AND b.user_role_id = c.id
        AND a.id = $1", 
        [id],
    |row| {
        Ok(FullUser {
            id: row.get(0)?,
            username: row.get(1)?,
            user_role: UserRole {
                id: row.get(2)?,
                name: row.get(3)?,
                num_borrowable_books: row.get(4)?,
            }
        })
    })
}

pub async fn add_user_to_db(
    State(state): State<AppState>,
    user: User,
    user_role_id: Uuid,
) -> Result<User, Error> {
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

pub async fn delete_user_from_db(State(state): State<AppState>, id: Uuid) -> Result<()> {
    state
        .db_pool
        .get()
        .unwrap()
        .execute("DELETE FROM users WHERE id = $1", [id])?;

    Ok(())
}

pub async fn list_user_roles_from_db(State(state): State<AppState>) -> Result<Vec<UserRole>> {
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

pub async fn get_user_role_from_db(State(state): State<AppState>, id: Uuid) -> Result<UserRole> {
    state.db_pool.get().unwrap().query_row(
        "SELECT * FROM user_roles WHERE user_roles.id = $1",
        [id],
        |row| {
            Ok(UserRole {
                id: row.get(0)?,
                name: row.get(1)?,
                num_borrowable_books: row.get(2)?,
            })
        },
    )
}

pub async fn add_user_role_to_db(State(state): State<AppState>, user_role: UserRole) -> Result<UserRole, Error> {
    let mut conn = state.db_pool.get().unwrap();

    let tx = conn.transaction()?;

    tx.execute(
        "INSERT INTO user_roles (id, name, num_borrowable_books) VALUES (?1, ?2, ?3)",
        (&user_role.id, &user_role.name, &user_role.num_borrowable_books),
    )?;

    tx.commit()?;

    Ok(user_role)
}

pub async fn delete_user_role_from_db(State(state): State<AppState>, id: Uuid) {

}
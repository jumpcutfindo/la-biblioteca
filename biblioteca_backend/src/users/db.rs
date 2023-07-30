use axum::extract::State;
use rusqlite::Result;
use uuid::Uuid;

use crate::AppState;

use super::model::User;

pub async fn get_all_users_from_db() {

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

pub async fn add_user_to_db() {

}

pub async fn delete_user_from_db() {
    
}
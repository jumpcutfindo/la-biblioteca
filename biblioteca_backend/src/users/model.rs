use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub user_role_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct UserRole {
    pub id: Uuid,
    pub name: String,
    pub num_borrowable_books: i32,
}

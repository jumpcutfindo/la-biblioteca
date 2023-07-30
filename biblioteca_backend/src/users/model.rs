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

#[derive(Debug, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub user_role_id: Uuid,
}



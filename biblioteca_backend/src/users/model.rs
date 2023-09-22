use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Deserialize)]
pub struct CreateUserRoleRequest {
    pub name: String,
    pub num_borrowable_books: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FullUser {
    pub id: Uuid,
    pub username: String,
    pub user_role: UserRole,
}

use serde::Deserialize;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct BorrowBookRequest {
    pub user_id: Uuid,
}
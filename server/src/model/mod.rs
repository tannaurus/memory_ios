use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod content;
pub use content::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: usize,
    pub user_id: usize,
    pub uuid: Uuid,
    pub title: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: usize,
    pub uuid: Uuid,
    pub name: String,
}

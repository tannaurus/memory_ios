use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod content;
pub use content::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub uuid: Uuid,
    pub title: String,
    pub content: Vec<Content>,
    pub created_at: String,
    pub updated_at: String,
}

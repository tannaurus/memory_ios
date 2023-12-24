use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

mod content;
pub use content::*;
use uuid::Uuid;

use crate::api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: u32,
    pub user_id: u32,
    pub uuid: Uuid,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<api::User> for User {
    fn into(self) -> api::User {
        api::User {
            uuid: self.uuid,
            name: self.name,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

pub struct Prompt {
    pub id: u32,
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<api::Prompt> for Prompt {
    fn into(self) -> api::Prompt {
        api::Prompt {
            uuid: self.uuid,
            name: self.name,
            description: self.description,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

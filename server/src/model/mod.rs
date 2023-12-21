use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod content;
pub use content::*;

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

impl Story {
    pub fn into_api_story(self, content: Vec<content::Content>) -> api::Story {
        let api_content = content.into_iter().map(|c| c.into()).collect();
        api::Story {
            uuid: self.uuid,
            title: self.title,
            content: api_content,
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub uuid: String,
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

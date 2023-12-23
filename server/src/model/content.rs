use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api;

#[derive(Debug, Clone, Serialize, sqlx::FromRow)]
pub struct Content {
    pub id: u32,
    pub story_id: u32,
    pub uuid: Uuid,
    pub kind: String,
    pub details: ContentDetails,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Into<api::Content> for Content {
    fn into(self) -> api::Content {
        api::Content {
            uuid: self.uuid,
            kind: self.kind,
            details: self.details.into(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum ContentDetails {
    Image(ImageContent),
    Text(TextContent),
}

impl Into<api::ContentDetails> for ContentDetails {
    fn into(self) -> api::ContentDetails {
        match self {
            ContentDetails::Image(image) => api::ContentDetails::Image(api::ImageContent {
                src: image.src,
                description: image.description,
            }),
            ContentDetails::Text(text) => api::ContentDetails::Text(api::TextContent {
                title: text.title,
                body: text.body,
            }),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    pub src: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    pub title: String,
    pub body: String,
}

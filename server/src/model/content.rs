use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::api;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: usize,
    pub story_id: usize,
    pub uuid: Uuid,
    pub content: ContentKind,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
pub enum ContentKind {
    Image(ImageContent),
    Text(TextContent),
}

impl Into<api::ContentKind> for ContentKind {
    fn into(self) -> api::ContentKind {
        match self {
            ContentKind::Image(image) => api::ContentKind::Image(api::ImageContent {
                src: image.src,
                description: image.description,
            }),
            ContentKind::Text(text) => api::ContentKind::Text(api::TextContent {
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

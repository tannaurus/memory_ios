use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub uuid: Uuid,
    pub title: String,
    pub content: Vec<Content>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
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

impl Into<model::ContentKind> for ContentKind {
    fn into(self) -> model::ContentKind {
        match self {
            ContentKind::Image(image) => model::ContentKind::Image(model::ImageContent {
                src: image.src,
                description: image.description,
            }),
            ContentKind::Text(text) => model::ContentKind::Text(model::TextContent {
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

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateStoryRequest {
    pub title: Option<String>,
    pub content: Option<Vec<UpdateContentRequest>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateContentRequest {
    pub uuid: Uuid,
    pub content: ContentKind,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model;

#[derive(Debug)]
pub enum ApiError {
    Encode,
    Decode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub uuid: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub uuid: Uuid,
    pub title: String,
    pub content: Vec<Content>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Story {
    pub fn new(story: model::Story, content: Vec<model::Content>) -> Self {
        let content = content.into_iter().map(|c| c.into()).collect();
        Self {
            uuid: story.uuid,
            title: story.title,
            content,
            created_at: story.created_at,
            updated_at: story.updated_at,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub uuid: Uuid,
    pub kind: String,
    pub details: ContentDetails,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[serde(rename_all = "snake_case")]
pub enum ContentDetails {
    Image(ImageContent),
    Text(TextContent),
}

impl ContentDetails {
    /// Returns the 'kind' [String] of the [ContentKind] variant.
    pub fn kind(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }

    pub fn details(&self) -> Result<String, ApiError> {
        Ok(serde_json::to_string(self).map_err(|_| ApiError::Encode)?)
    }
}

impl Into<model::ContentDetails> for ContentDetails {
    fn into(self) -> model::ContentDetails {
        match self {
            ContentDetails::Image(image) => model::ContentDetails::Image(model::ImageContent {
                src: image.src,
                description: image.description,
            }),
            ContentDetails::Text(text) => model::ContentDetails::Text(model::TextContent {
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
    pub content: ContentDetails,
}

#[derive(Debug, Clone, Serialize)]
pub struct Prompt {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

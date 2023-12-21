use serde::{Deserialize, Serialize};
use uuid::Uuid;

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

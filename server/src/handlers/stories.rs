use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{read_db, DbEntity},
    AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Story {
    title: String,
    preview: Content,
    content: Vec<Content>,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
pub(crate) enum Content {
    Image(ImageContent),
    Text(TextContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct ImageContent {
    src: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct TextContent {
    title: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GetStoriesResponse {
    stories: Vec<Story>,
}

pub(crate) async fn get_stories() -> Result<Json<GetStoriesResponse>, AppError> {
    let story_one = read_db(DbEntity::Stories, "1")?;
    let story_two = read_db(DbEntity::Stories, "2")?;

    let response = GetStoriesResponse {
        stories: vec![story_one, story_two],
    };

    Ok(Json(response))
}
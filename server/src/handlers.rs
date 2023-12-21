use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{self, DbEntity},
    AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Prompt {
    name: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct GetPromptsResponse {
    prompts: Vec<Prompt>,
}

pub(crate) async fn get_prompts() -> Result<Json<GetPromptsResponse>, AppError> {
    let mocked_data = utils::read_db(DbEntity::Prompts, "mock.json")?;

    let response = GetPromptsResponse {
        prompts: mocked_data,
    };

    Ok(Json(response))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct User {
    name: String,
    picture: String,
    followers: u32,
    following: u32,
    bio: String,
}

pub(crate) async fn get_user() -> Result<Json<User>, AppError> {
    let mocked_data = utils::read_db(DbEntity::Users, "mock.json")?;

    Ok(Json(mocked_data))
}

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
    let mocked_data = utils::read_db(DbEntity::Stories, "mock.json")?;

    let response = GetStoriesResponse {
        stories: mocked_data,
    };

    Ok(Json(response))
}

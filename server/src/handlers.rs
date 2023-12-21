use axum::Json;
use serde::{Deserialize, Serialize};

use crate::AppError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct Prompt {
    name: String,
    description: String,
}

#[derive(Debug, Clone, Serialize)]
pub(crate) struct GetPromptsResponse {
    prompts: Vec<Prompt>,
}

pub(crate) async fn get_prompts() -> Result<Json<GetPromptsResponse>, AppError> {
    let prompt_one = Prompt {
        name: String::from("Daily"),
        description: String::from("Write about your day"),
    };

    let prompt_two = Prompt {
        name: String::from("Reflect"),
        description: String::from("Take a step back"),
    };

    let response = GetPromptsResponse {
        prompts: vec![prompt_one, prompt_two],
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
    let user = User {
        name: String::from("Tanner"),
        picture: String::from("profile"),
        followers: 434,
        following: 64,
        bio: String::from("Developer @ 1Password"),
    };

    Ok(Json(user))
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
    let mexico_city = Story {
        title: String::from("Mexico City"),
        preview: Content::Image(ImageContent {
            src: String::from("mexico_city"),
            description: String::new(),
        }),
        content: Vec::new(),
        created_at: String::from(""),
        updated_at: String::from(""),
    };

    let response = GetStoriesResponse {
        stories: vec![mexico_city],
    };

    Ok(Json(response))
}

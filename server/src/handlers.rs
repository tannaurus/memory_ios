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
    println!("GET /prompts");

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
    println!("GET /user");

    let user = User {
        name: String::from("Tanner"),
        picture: String::from("profile"),
        followers: 434,
        following: 64,
        bio: String::from("Developer @ 1Password"),
    };

    Ok(Json(user))
}

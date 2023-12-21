use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    utils::{self, DbEntity},
    AppError,
};

pub mod stories;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    name: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptsResponse {
    prompts: Vec<Prompt>,
}

pub async fn get_prompts() -> Result<Json<GetPromptsResponse>, AppError> {
    let prompt_one = utils::read_db(DbEntity::Prompts, "1")?;
    let prompt_two = utils::read_db(DbEntity::Prompts, "2")?;
    let prompt_three = utils::read_db(DbEntity::Prompts, "3")?;

    let response = GetPromptsResponse {
        prompts: vec![prompt_one, prompt_two, prompt_three],
    };

    Ok(Json(response))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    name: String,
    picture: String,
    followers: u32,
    following: u32,
    bio: String,
}

pub async fn get_user() -> Result<Json<User>, AppError> {
    let mocked_data = utils::read_db(DbEntity::Users, "1")?;

    Ok(Json(mocked_data))
}

use axum::Json;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    access::{self, DbEntity},
    AppError,
};

pub mod stories;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompt {
    uuid: Uuid,
    name: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPromptsResponse {
    prompts: Vec<Prompt>,
}

pub async fn get_prompts() -> Result<Json<GetPromptsResponse>, AppError> {
    let prompt_one =
        access::select_with_uuid(DbEntity::Prompts, "98b69e15-fcde-40dd-a7d0-1072058cf25f")?;
    let prompt_two =
        access::select_with_uuid(DbEntity::Prompts, "35d359cc-899c-4cd9-a8e8-2726192f4e71")?;
    let prompt_three =
        access::select_with_uuid(DbEntity::Prompts, "cfbc81b0-8781-4854-8f0e-fc83769553ae")?;

    let response = GetPromptsResponse {
        prompts: vec![prompt_one, prompt_two, prompt_three],
    };

    Ok(Json(response))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    uuid: Uuid,
    name: String,
    picture: String,
    followers: u32,
    following: u32,
    bio: String,
}

pub async fn get_user() -> Result<Json<User>, AppError> {
    let mocked_data =
        access::select_with_uuid(DbEntity::Users, "6c81e345-1ab3-463b-8aa2-916da81c1d0c")?;

    Ok(Json(mocked_data))
}

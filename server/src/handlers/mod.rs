use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{access::user::AccessUser, api, AppContext, AppError};

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

pub async fn get_prompts() -> Result<Json<()>, AppError> {
    // let prompt_one =
    //     access::select_with_uuid(DbEntity::Prompts, "98b69e15-fcde-40dd-a7d0-1072058cf25f")?;
    // let prompt_two =
    //     access::select_with_uuid(DbEntity::Prompts, "35d359cc-899c-4cd9-a8e8-2726192f4e71")?;
    // let prompt_three =
    //     access::select_with_uuid(DbEntity::Prompts, "cfbc81b0-8781-4854-8f0e-fc83769553ae")?;

    // let response = GetPromptsResponse {
    //     prompts: vec![prompt_one, prompt_two, prompt_three],
    // };

    // Ok(Json(response))

    Ok(Json(()))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

pub async fn create_user(
    ctx: State<AppContext>,
    request: Json<CreateUserRequest>,
) -> Result<Json<api::User>, AppError> {
    let user = ctx.db.create_user(request.name.clone()).await?;

    Ok(Json(user.into()))
}

pub async fn get_user(
    ctx: State<AppContext>,
    uuid: Path<Uuid>,
) -> Result<Json<api::User>, AppError> {
    let user = ctx.db.get_user(uuid.0).await?;

    Ok(Json(user.into()))
}

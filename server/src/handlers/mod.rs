use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{access::user::AccessUser, action, api, AppContext, AppError};

pub mod stories;

#[derive(Debug, Clone, Serialize)]
pub struct GetPromptsResponse {
    prompts: Vec<api::Prompt>,
}

pub async fn get_prompts(ctx: State<AppContext>) -> Result<Json<GetPromptsResponse>, AppError> {
    let prompts = action::prompts::get_prompts(ctx.db.clone())
        .await?
        .into_iter()
        .map(|p| p.into())
        .collect();

    Ok(Json(GetPromptsResponse { prompts }))
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

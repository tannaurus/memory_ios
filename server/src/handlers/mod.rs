use axum::{extract::State, Json};
use serde::Serialize;

use crate::{action, api, AppContext, AppError};

pub mod story;
pub mod user;

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

use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    access::{self, DbEntity},
    api, model, AppContext, AppError,
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
pub struct CreateUserRequest {
    pub name: String,
}

pub async fn create_user(
    ctx: State<AppContext>,
    request: Json<CreateUserRequest>,
) -> Result<Json<api::User>, AppError> {
    let uuid = Uuid::new_v4();
    let result = sqlx::query!(
        "INSERT INTO users (name, uuid) VALUES (?, ?)",
        request.name,
        uuid.to_string()
    )
    .execute(&ctx.db)
    .await
    .map_err(|e| {
        println!("{}", e);
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to write user to database.".into(),
        )
    })?;

    let user = sqlx::query_as!(
        model::User,
        "SELECT * FROM users WHERE id = ?",
        result.last_insert_id()
    )
    .fetch_one(&ctx.db)
    .await
    .map_err(|_| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to write user to database.".into(),
        )
    })?;

    Ok(Json(api::User {
        name: user.name,
        uuid: user.uuid,
        created_at: user.created_at,
        updated_at: user.updated_at,
    }))
}

pub async fn get_user() -> Result<Json<api::User>, AppError> {
    let mocked_data =
        access::select_with_uuid(DbEntity::Users, "6c81e345-1ab3-463b-8aa2-916da81c1d0c")?;

    Ok(Json(mocked_data))
}

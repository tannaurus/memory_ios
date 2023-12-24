use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{access::story::AccessStory, action, api, AppContext, AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoryRequest {
    title: String,
    content: Vec<api::ContentDetails>,
}

pub async fn handle_create_story(
    ctx: State<AppContext>,
    request: Json<CreateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let story =
        action::create_story(&ctx.db, request.title.clone(), request.content.clone()).await?;

    Ok(Json(story))
}

pub async fn handle_get_story(
    ctx: State<AppContext>,
    Path(story_uuid): Path<Uuid>,
) -> Result<Json<api::Story>, AppError> {
    let story = action::get_story(&ctx.db, story_uuid).await?;
    Ok(Json(story))
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStoryRequest {
    title: Option<String>,
    content: Vec<UpdateContentRequest>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct UpdateContentRequest {
    uuid: Uuid,
    content: api::ContentDetails,
}

impl Into<action::ContentUpdate> for UpdateContentRequest {
    fn into(self) -> action::ContentUpdate {
        action::ContentUpdate {
            uuid: self.uuid,
            content: self.content,
        }
    }
}

pub async fn handle_update_story(
    ctx: State<AppContext>,
    story_uuid: Path<Uuid>,
    request: Json<UpdateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let story = ctx.db.get_story_by_uuid(story_uuid.0).await?;

    action::update_story(&ctx.db, story.clone(), request.title.clone(), None).await?;

    let content_updates = request
        .content
        .clone()
        .into_iter()
        .map(|u| u.into())
        .collect();

    action::update_content(&ctx.db, &story, content_updates).await?;

    let story = action::get_story(&ctx.db, story_uuid.0).await?;

    Ok(Json(story))
}

pub async fn handle_delete_story(
    ctx: State<AppContext>,
    story_uuid: Path<Uuid>,
) -> Result<Json<()>, AppError> {
    let response = action::delete_story(&ctx.db, story_uuid.0).await?;
    Ok(Json(response))
}

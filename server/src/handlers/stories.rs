use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    access,
    action::{self, StoryUpdate},
    api, AppContext, AppError,
};

use crate::model;

pub async fn handle_get_story(Path(story_uuid): Path<Uuid>) -> Result<Json<api::Story>, AppError> {
    let story = find_story_by_uuid(story_uuid)?;
    Ok(Json(story))
}

fn find_story_by_uuid(story_uuid: Uuid) -> Result<api::Story, AppError> {
    let story: model::Story =
        access::select_with_uuid(access::DbEntity::Stories, &story_uuid.to_string())?;

    if story.deleted {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "Story has been deleted.".into(),
        ));
    }

    let content = access::select_all_by_id_column(access::DbEntity::Content, story.id, "story_id")?;

    Ok(story.into_api_story(content))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoryRequest {
    title: String,
    content: Vec<api::ContentKind>,
}

pub async fn handle_create_story(
    ctx: State<AppContext>,
    request: Json<CreateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let story = create_story(ctx.auth.user.id, request.0)?;

    sqlx::query!(
        r#"INSERT INTO stories (uuid, user_id, title) VALUES (?, ?, ?)"#,
        story.uuid.to_string(),
        ctx.auth.user.id,
        story.title
    )
    .execute(&ctx.db)
    .await
    .map_err(|_| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to insert story".into(),
        )
    })?;

    Ok(Json(story))
}

fn create_story(user_id: u32, request: CreateStoryRequest) -> Result<api::Story, AppError> {
    let now = Utc::now();

    let uuid = Uuid::new_v4();
    let story_id = access::generate_entity_id(access::DbEntity::Stories)?;
    let story = model::Story {
        id: story_id,
        uuid,
        user_id,
        title: request.title.clone(),
        created_at: now.clone(),
        updated_at: now.clone(),
        deleted: false,
    };

    access::write_db(access::DbEntity::Stories, &uuid.to_string(), &story)?;

    // generate db metadata and write to db
    let content = request
        .content
        .clone()
        .into_iter()
        .map(|c| {
            let uuid = Uuid::new_v4();
            let id = access::generate_entity_id(access::DbEntity::Content).unwrap();
            let model_content = model::Content {
                id,
                story_id,
                uuid,
                created_at: now.clone(),
                updated_at: now.clone(),
                content: c.clone().into(),
            };

            access::write_db(access::DbEntity::Content, &uuid.to_string(), &model_content).unwrap();

            model_content
        })
        .collect();

    Ok(story.into_api_story(content))
}

pub async fn handle_update_story(
    story_uuid: Path<Uuid>,
    request: Json<api::UpdateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let story: model::Story =
        access::select_with_uuid(access::DbEntity::Stories, &story_uuid.0.to_string())?;

    let story_updates = action::StoryUpdate {
        title: request.title.clone(),
        deleted: None,
    };

    let updated_story = action::update_story(story, story_updates)?;

    let mut updated_content = Vec::new();
    if let Some(content_updates) = request.content.clone() {
        let content: Vec<model::Content> = access::select_all_by_id_column(
            access::DbEntity::Content,
            updated_story.id,
            "story_id",
        )?;
        let content_map = content.into_iter().map(|c| (c.uuid, c)).collect();
        let updates = content_updates
            .into_iter()
            .map(|update| action::ContentUpdate {
                uuid: update.uuid,
                content: update.content,
            })
            .collect();

        updated_content = action::update_content(&updated_story, content_map, updates)?;
    }

    access::update(
        access::DbEntity::Stories,
        &updated_story.uuid.to_string(),
        &updated_story,
    )?;

    for content_update in updated_content.into_iter() {
        access::update(
            access::DbEntity::Content,
            &content_update.uuid.to_string(),
            &content_update.content,
        )?
    }

    let content: Vec<model::Content> =
        access::select_all_by_id_column(access::DbEntity::Content, updated_story.id, "story_id")?;

    let story = updated_story.into_api_story(content);

    Ok(Json(story))
}

pub async fn handle_delete_story(story_uuid: Path<Uuid>) -> Result<Json<()>, AppError> {
    let response = delete_story(story_uuid.0)?;
    Ok(Json(response))
}

fn delete_story(story_uuid: Uuid) -> Result<(), AppError> {
    let story_uuid = story_uuid.to_string();
    let story: model::Story = access::select_with_uuid(access::DbEntity::Stories, &story_uuid)?;

    let story = action::update_story(
        story,
        StoryUpdate {
            title: None,
            deleted: Some(true),
        },
    )?;

    access::update(access::DbEntity::Stories, &story_uuid, &story)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::api;

    fn build_story_request() -> super::CreateStoryRequest {
        let content = api::ContentKind::Text(api::TextContent {
            title: "A day in the life".into(),
            body: "A picnic".into(),
        });
        super::CreateStoryRequest {
            title: "Hello, world 👋".into(),
            content: vec![content],
        }
    }

    #[test]
    fn can_create_story() {
        // Test
        let request = build_story_request();
        super::create_story(1, request).unwrap();
    }

    #[test]
    fn can_delete_story() {
        // Set up
        let request = build_story_request();
        let created_story = super::create_story(1, request).unwrap();

        // Test
        super::delete_story(created_story.uuid).unwrap();
    }

    #[test]
    fn can_get_story() {
        // Set up
        let request = build_story_request();
        let created_story = super::create_story(1, request).unwrap();

        // Test
        let response = super::find_story_by_uuid(created_story.uuid).unwrap();
        assert_eq!(response.content.len(), 1);
    }

    #[test]
    fn cant_get_deleted_story() {
        // Set up
        let request = build_story_request();
        let created_story = super::create_story(1, request).unwrap();

        super::delete_story(created_story.uuid).unwrap();

        let response = super::find_story_by_uuid(created_story.uuid).unwrap_err();
        assert_eq!(response.0, StatusCode::BAD_REQUEST);
    }
}

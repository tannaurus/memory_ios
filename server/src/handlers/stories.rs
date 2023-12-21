use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use axum_macros::debug_handler;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{access, action, api, auth::AuthState, AppError};

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

    let story = api::Story {
        uuid: story.uuid,
        title: story.title,
        content,
        created_at: story.created_at,
        updated_at: story.updated_at,
    };

    Ok(story)
}

// #[derive(Serialize, Deserialize)]
// pub struct GetStoriesResponse {
//     pub stories: Vec<api::Story>,
// }

// pub async fn handle_get_stories(
//     Path(story_uuid): Path<Uuid>,
// ) -> Result<Json<GetStoriesResponse>, AppError> {
//     let response = get_stories(story_uuid)?;
//     Ok(Json(response))
// }

// fn get_stories(_story_uuid: Uuid) -> Result<GetStoriesResponse, AppError> {
//     let response = GetStoriesResponse {
//         stories: Vec::new(),
//     };
//     Ok(response)
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoryRequest {
    title: String,
    content: Vec<api::ContentKind>,
}

#[debug_handler]
pub async fn handle_create_story(
    auth_state: State<AuthState>,
    request: Json<CreateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let story = create_story(auth_state.0.user.id, request.0)?;
    Ok(Json(story))
}

fn create_story(user_id: usize, request: CreateStoryRequest) -> Result<api::Story, AppError> {
    let now = Utc::now().to_rfc3339();

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

            api::Content {
                uuid: model_content.uuid,
                content: c,
                created_at: model_content.created_at,
                updated_at: model_content.updated_at,
            }
        })
        .collect();

    Ok(api::Story {
        uuid: story.uuid,
        title: story.title,
        content,
        created_at: story.created_at,
        updated_at: story.updated_at,
    })
}

pub async fn handle_update_story(
    story_uuid: Path<Uuid>,
    request: Json<api::UpdateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let updated_story = action::update_story(story_uuid.0, request.0)?;
    Ok(Json(updated_story))
}

pub async fn handle_delete_story(story_uuid: Path<Uuid>) -> Result<Json<()>, AppError> {
    let response = delete_story(story_uuid.0)?;
    Ok(Json(response))
}

fn delete_story(story_uuid: Uuid) -> Result<(), AppError> {
    let story_uuid = story_uuid.to_string();
    let mut story: model::Story = access::select_with_uuid(access::DbEntity::Stories, &story_uuid)?;
    if story.deleted {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "Story has already been deleted.".into(),
        ));
    }
    story.deleted = true;
    access::update(access::DbEntity::Stories, &story_uuid, &story)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;

    use crate::{action, api};

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

    #[test]
    fn can_update_story_title() {
        // Set up
        let create_request = build_story_request();
        let created_story = super::create_story(1, create_request).unwrap();

        // Test
        let updated_title = "Goodbye, moon!".to_string();
        let request = api::UpdateStoryRequest {
            title: Some(updated_title.clone()),
            content: None,
        };

        let updated_story = action::update_story(created_story.uuid, request).unwrap();
        assert_eq!(updated_story.title, updated_title);
    }

    #[test]
    fn can_update_story_content() {
        // Set up
        let create_request = build_story_request();
        let created_story = super::create_story(1, create_request).unwrap();

        // Test
        let title = "Goodbye, moon!".to_string();
        let body = "It's a long way home".to_string();
        let updated_content = api::UpdateContentRequest {
            uuid: created_story.content[0].uuid,
            content: api::ContentKind::Text(api::TextContent {
                title: title.clone(),
                body,
            }),
        };

        let request = api::UpdateStoryRequest {
            title: None,
            content: Some(vec![updated_content]),
        };

        let updated_story = action::update_story(created_story.uuid, request).unwrap();
        match updated_story.content[0].clone().content {
            api::ContentKind::Text(story) => assert_eq!(story.title, title),
            _ => panic!("Bad ending"),
        }
    }
}

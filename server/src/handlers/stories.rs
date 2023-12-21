use axum::{extract::Path, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    utils::{read_db, write_db, DbEntity},
    AppError,
};

use crate::models;

pub async fn get_story(Path(story_uuid): Path<Uuid>) -> Result<Json<models::Story>, AppError> {
    let story = read_db(DbEntity::Stories, &story_uuid.to_string())?;

    Ok(Json(story))
}

#[derive(Serialize, Deserialize)]
pub struct GetStoriesResponse {
    pub stories: Vec<models::Story>,
}

pub async fn get_stories() -> Result<Json<GetStoriesResponse>, AppError> {
    let response = GetStoriesResponse {
        stories: Vec::new(),
    };
    Ok(Json(response))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoryRequest {
    title: String,
    content: Vec<models::ContentKind>,
}

pub async fn create_story(
    request: Json<CreateStoryRequest>,
) -> Result<Json<models::Story>, AppError> {
    let now = Utc::now().to_rfc3339();

    // generate db metadata and write to db
    let content: Vec<models::Content> = request
        .content
        .clone()
        .into_iter()
        .map(|c| {
            let uuid = Uuid::new_v4();
            let c = models::Content {
                uuid,
                created_at: now.clone(),
                updated_at: now.clone(),
                content: c,
            };
            // unwrapping here to not going to overthink temporary code.
            write_db(DbEntity::Content, &uuid.to_string(), &c).unwrap();
            c
        })
        .collect();

    let uuid = Uuid::new_v4();
    let story = models::Story {
        uuid,
        title: request.title.clone(),
        content,
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    write_db(DbEntity::Stories, &uuid.to_string(), &story)?;

    Ok(Json(story))
}

// pub async fn patch_story(request: Json<CreateStoryRequest>) -> Result<Json<Story>, AppError> {

// }

#[cfg(test)]
mod tests {
    use std::fs;

    use axum::Json;
    use uuid::Uuid;

    use crate::models::{ContentKind, TextContent};

    use super::{create_story, get_stories, CreateStoryRequest};

    fn build_story_request() -> CreateStoryRequest {
        let content = ContentKind::Text(TextContent {
            title: "A day in the life".into(),
            body: "A picnic".into(),
        });
        CreateStoryRequest {
            title: "Hello, world 👋".into(),
            content: vec![content],
        }
    }

    fn clean_up_story(story_uuid: Uuid) {
        let story_location = format!("db/stories/{}.json", story_uuid);
        fs::remove_file(story_location).unwrap()
    }

    #[tokio::test]
    async fn can_create_story() {
        let request = build_story_request();
        let story = create_story(Json(request)).await.unwrap();

        clean_up_story(story.uuid)
    }

    #[tokio::test]
    async fn can_get_stories() {
        let request = build_story_request();
        let created_story = create_story(Json(request)).await.unwrap();

        let response = get_stories().await.unwrap();

        assert_eq!(response.stories.len(), 0);

        clean_up_story(created_story.uuid)
    }
}

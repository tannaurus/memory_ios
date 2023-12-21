use axum::{extract::Path, Json};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    utils::{read_db, write_db, DbEntity},
    AppError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    uuid: Uuid,
    title: String,
    preview: Content,
    content: Vec<Content>,
    created_at: String,
    updated_at: String,
}

impl Into<StoryPreview> for Story {
    fn into(self) -> StoryPreview {
        StoryPreview {
            uuid: self.uuid,
            preview: self.preview,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
#[serde(rename_all = "snake_case")]
pub enum Content {
    Image(ImageContent),
    Text(TextContent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageContent {
    src: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    title: String,
    description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoryPreview {
    uuid: Uuid,
    preview: Content,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetStoriesResponse {
    stories: Vec<StoryPreview>,
}

pub async fn get_stories_previews() -> Result<Json<GetStoriesResponse>, AppError> {
    let story_one = read_db(DbEntity::Stories, "e76ba6b7-2eda-4edc-b913-fb8736e62a28")?;
    let story_two = read_db(DbEntity::Stories, "ff9a0564-8a50-4b37-a5de-cc1f41bf178d")?;

    let response = GetStoriesResponse {
        stories: vec![story_one, story_two],
    };

    Ok(Json(response))
}

pub async fn get_story(Path(story_uuid): Path<Uuid>) -> Result<Json<Story>, AppError> {
    let story = read_db(DbEntity::Stories, &story_uuid.to_string())?;

    Ok(Json(story))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoryRequest {
    title: String,
    preview: Content,
    content: Vec<Content>,
}

pub async fn create_story(request: Json<CreateStoryRequest>) -> Result<Json<Story>, AppError> {
    let uuid = Uuid::new_v4();
    let created_at = Utc::now().to_rfc3339();
    let updated_at = created_at.clone();
    let story = Story {
        uuid,
        title: request.title.clone(),
        preview: request.preview.clone(),
        content: request.content.clone(),
        created_at,
        updated_at,
    };
    write_db(DbEntity::Stories, &uuid.to_string(), &story)?;

    Ok(Json(story))
}

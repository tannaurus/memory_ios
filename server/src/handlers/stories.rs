use axum::{extract::Path, Json};
use axum_macros::debug_handler;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    access::{self, select_all_by_id_column, select_with_uuid, write_db, DbEntity},
    api, AppError,
};

use crate::model;

pub async fn handle_get_story(Path(story_uuid): Path<Uuid>) -> Result<Json<api::Story>, AppError> {
    let story = find_story_by_uuid(story_uuid)?;
    Ok(Json(story))
}

fn find_story_by_uuid(story_uuid: Uuid) -> Result<api::Story, AppError> {
    let story: model::Story = select_with_uuid(DbEntity::Stories, &story_uuid.to_string())?;

    let content = select_all_by_id_column(DbEntity::Content, story.id, "story_id")?;

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

pub async fn handle_create_story(
    request: Json<CreateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let story = create_story(request.0)?;
    Ok(Json(story))
}

fn create_story(request: CreateStoryRequest) -> Result<api::Story, AppError> {
    let now = Utc::now().to_rfc3339();

    let uuid = Uuid::new_v4();
    let story_id = access::generate_entity_id(DbEntity::Stories)?;
    let story = model::Story {
        id: story_id,
        uuid,
        title: request.title.clone(),
        created_at: now.clone(),
        updated_at: now.clone(),
    };

    write_db(DbEntity::Stories, &uuid.to_string(), &story)?;

    // generate db metadata and write to db
    let content = request
        .content
        .clone()
        .into_iter()
        .map(|c| {
            let uuid = Uuid::new_v4();
            let id = access::generate_entity_id(DbEntity::Content).unwrap();
            let model_content = model::Content {
                id,
                story_id,
                uuid,
                created_at: now.clone(),
                updated_at: now.clone(),
                content: c.clone().into(),
            };
            write_db(DbEntity::Content, &uuid.to_string(), &model_content).unwrap();

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

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateStoryRequest {
    pub title: Option<String>,
    pub content: Option<Vec<UpdateContentRequest>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UpdateContentRequest {
    pub uuid: Uuid,
    pub content: api::ContentKind,
}

#[debug_handler]
pub async fn handle_update_story(
    Path(story_uuid): Path<Uuid>,
    request: Json<UpdateStoryRequest>,
) -> Result<Json<api::Story>, AppError> {
    let updated_story = update_story(story_uuid, request.0)?;
    Ok(Json(updated_story))
}

fn update_story(story_uuid: Uuid, request: UpdateStoryRequest) -> Result<api::Story, AppError> {
    let mut story: model::Story = select_with_uuid(DbEntity::Stories, &story_uuid.to_string())?;
    if let Some(title) = request.title.clone() {
        story.title = title;
    }

    for content_update in request.content.unwrap_or(Vec::new()).into_iter() {
        let existing_content: model::Content =
            select_with_uuid(DbEntity::Content, &content_update.uuid.to_string())?;
        let updated_content = model::Content {
            content: content_update.content.into(),
            ..existing_content
        };
        access::update::<model::Content>(
            DbEntity::Content,
            &updated_content.uuid.to_string(),
            &updated_content,
        )?;
    }

    let content: Vec<model::Content> =
        select_all_by_id_column(DbEntity::Content, story.id, "story_id")?;
    let content: Vec<api::Content> = content
        .into_iter()
        .map(|c| api::Content {
            uuid: c.uuid,
            content: c.content.into(),
            created_at: c.created_at,
            updated_at: c.updated_at,
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

#[cfg(test)]
mod tests {
    use std::{thread::sleep, time::Duration};

    use axum::Json;

    use crate::api::{self};

    fn build_story_request() -> super::CreateStoryRequest {
        sleep(Duration::from_secs(2));
        let content = api::ContentKind::Text(api::TextContent {
            title: "A day in the life".into(),
            body: "A picnic".into(),
        });
        super::CreateStoryRequest {
            title: "Hello, world 👋".into(),
            content: vec![content],
        }
    }

    // fn clean_up() {
    //     fs::remove_dir_all("db/stories").unwrap();
    //     fs::create_dir("db/stories").unwrap();
    //     fs::remove_dir_all("db/content").unwrap();
    //     fs::create_dir("db/content").unwrap()
    // }

    #[test]
    fn can_create_story() {
        // Test
        let request = build_story_request();
        super::create_story(request).unwrap();
    }

    #[tokio::test]
    async fn can_get_story() {
        // Set up
        let request = build_story_request();
        let created_story = super::handle_create_story(Json(request)).await.unwrap();

        // Test
        let response = super::find_story_by_uuid(created_story.uuid).unwrap();
        println!("{:?}", response);
        assert_eq!(response.content.len(), 1);
    }

    #[test]
    fn can_update_story_title() {
        // Set up
        let create_request = build_story_request();
        let created_story = super::create_story(create_request).unwrap();

        // Test
        let updated_title = "Goodbye, moon!".to_string();
        let request = super::UpdateStoryRequest {
            title: Some(updated_title.clone()),
            content: None,
        };

        let updated_story = super::update_story(created_story.uuid, request).unwrap();
        assert_eq!(updated_story.title, updated_title);
    }

    #[test]
    fn can_update_story_content() {
        // Set up
        let create_request = build_story_request();
        let created_story = super::create_story(create_request).unwrap();

        // Test
        let title = "Goodbye, moon!".to_string();
        let body = "It's a long way home".to_string();
        let updated_content = super::UpdateContentRequest {
            uuid: created_story.content[0].uuid,
            content: api::ContentKind::Text(api::TextContent {
                title: title.clone(),
                body,
            }),
        };

        let request = super::UpdateStoryRequest {
            title: None,
            content: Some(vec![updated_content]),
        };

        let updated_story = super::update_story(created_story.uuid, request).unwrap();
        match updated_story.content[0].clone().content {
            api::ContentKind::Text(story) => assert_eq!(story.title, title),
            _ => panic!("Bad ending"),
        }
    }
}

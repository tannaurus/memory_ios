use std::collections::HashMap;

use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

use crate::{api, model, AppError};

pub struct StoryUpdate {
    pub title: Option<String>,
    pub deleted: Option<bool>,
}

pub fn update_story(
    mut story: model::Story,
    updates: StoryUpdate,
) -> Result<model::Story, AppError> {
    if story.deleted {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "Story has been deleted.".into(),
        ));
    }

    let now = Utc::now().to_rfc3339();

    if let Some(title) = updates.title.clone() {
        story.title = title;
        story.updated_at = now.clone();
    }

    if let Some(delete) = updates.deleted.clone() {
        story.deleted = delete;
        story.updated_at = now.clone();
    }

    Ok(story)
}

pub struct ContentUpdate {
    pub uuid: Uuid,
    pub content: api::ContentKind,
}

pub fn update_content(
    story: &model::Story,
    mut content: HashMap<Uuid, model::Content>,
    updates: Vec<ContentUpdate>,
) -> Result<Vec<model::Content>, AppError> {
    if story.deleted {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "Story has been deleted.".into(),
        ));
    }

    let now = Utc::now().to_rfc3339();

    for content_update in updates.into_iter() {
        let existing_content = content
            .get(&content_update.uuid)
            .ok_or(AppError(StatusCode::BAD_REQUEST, "Missing content".into()))?
            .clone();

        let updated_content = model::Content {
            content: content_update.content.into(),
            updated_at: now.clone(),
            ..existing_content
        };

        content.insert(existing_content.uuid, updated_content);
    }

    let response: Vec<model::Content> = content.into_values().collect();

    Ok(response)
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, str::FromStr};
    use uuid::Uuid;

    use crate::{api, model};

    #[test]
    fn can_update_story_title() {
        // Set up
        let story = model::Story {
            id: 1,
            user_id: 1,
            uuid: Uuid::from_str("7d18fe04-cc45-41ec-b899-1dd54bcfcf0b").unwrap(),
            title: "Hello, world!".into(),
            created_at: "2023-12-19T21:04:45.976885+00:00".into(),
            updated_at: "2023-12-19T21:04:45.976885+00:00".into(),
            deleted: false,
        };

        // Test
        let updated_title = "Goodbye, moon!".to_string();
        let updates = super::StoryUpdate {
            title: Some(updated_title.clone()),
            deleted: None,
        };

        let updated_story = super::update_story(story.clone(), updates).unwrap();
        assert_eq!(updated_story.title, updated_title);
        assert_ne!(story.title, updated_story.title);
        assert_ne!(story.updated_at, updated_story.updated_at);
    }

    #[test]
    fn can_delete_story() {
        // Set up
        let story = model::Story {
            id: 1,
            user_id: 1,
            uuid: Uuid::from_str("7d18fe04-cc45-41ec-b899-1dd54bcfcf0b").unwrap(),
            title: "Hello, world!".into(),
            created_at: "2023-12-19T21:04:45.976885+00:00".into(),
            updated_at: "2023-12-19T21:04:45.976885+00:00".into(),
            deleted: false,
        };

        // Test
        let updates = super::StoryUpdate {
            title: None,
            deleted: Some(true),
        };

        let updated_story = super::update_story(story.clone(), updates).unwrap();
        assert_eq!(story.deleted, false);
        assert_eq!(updated_story.deleted, true);
        assert_ne!(story.updated_at, updated_story.updated_at);
    }

    #[test]
    fn can_update_story_content() {
        // Set up
        let story = model::Story {
            id: 1,
            user_id: 1,
            uuid: Uuid::from_str("7d18fe04-cc45-41ec-b899-1dd54bcfcf0b").unwrap(),
            title: "Hello, world!".into(),
            created_at: "2023-12-19T21:04:45.976885+00:00".into(),
            updated_at: "2023-12-19T21:04:45.976885+00:00".into(),
            deleted: false,
        };

        let content = model::Content {
            id: 1,
            story_id: 1,
            uuid: Uuid::from_str("56f2b949-f788-4146-a216-671d28d9acbf").unwrap(),
            content: model::ContentKind::Text(model::TextContent {
                title: "Hello, world!".into(),
                body: "Something note worthy".into(),
            }),
            created_at: "2023-12-19T21:04:45.976885+00:00".into(),
            updated_at: "2023-12-19T21:04:45.976885+00:00".into(),
        };

        // Test
        let title = "Goodbye, moon!".to_string();
        let body = "It's a long way home".to_string();
        let updates = vec![super::ContentUpdate {
            uuid: content.uuid.clone(),
            content: api::ContentKind::Text(api::TextContent {
                title: title.clone(),
                body: body.clone(),
            }),
        }];

        let mut content_map = HashMap::new();
        content_map.insert(content.uuid, content);

        let updated_content = super::update_content(&story, content_map, updates).unwrap();
        match updated_content[0].clone().content {
            model::ContentKind::Text(story) => {
                assert_eq!(story.title, title);
                assert_eq!(story.body, body);
            }
            _ => panic!("Bad ending"),
        }
    }
}

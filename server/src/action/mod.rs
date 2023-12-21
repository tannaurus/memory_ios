use axum::http::StatusCode;
use uuid::Uuid;

use crate::{access, api, model, AppError};

pub fn update_story(
    story_uuid: Uuid,
    request: api::UpdateStoryRequest,
) -> Result<api::Story, AppError> {
    let story_uuid = story_uuid.to_string();
    let mut story: model::Story = access::select_with_uuid(access::DbEntity::Stories, &story_uuid)?;
    if story.deleted {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "Story has been deleted.".into(),
        ));
    }
    if let Some(title) = request.title.clone() {
        story.title = title;
    }

    access::update(access::DbEntity::Stories, &story_uuid, &story)?;

    for content_update in request.content.unwrap_or(Vec::new()).into_iter() {
        let existing_content: model::Content =
            access::select_with_uuid(access::DbEntity::Content, &content_update.uuid.to_string())?;
        let updated_content = model::Content {
            content: content_update.content.into(),
            ..existing_content
        };
        access::update::<model::Content>(
            access::DbEntity::Content,
            &updated_content.uuid.to_string(),
            &updated_content,
        )?;
    }

    let content: Vec<model::Content> =
        access::select_all_by_id_column(access::DbEntity::Content, story.id, "story_id")?;
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

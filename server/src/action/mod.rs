use axum::http::StatusCode;
use uuid::Uuid;

use crate::{
    access::{self, AccessError},
    api,
    auth::VerifiedUser,
    model, AppError,
};

pub mod prompts;

pub enum ActionError {
    AccessError(access::AccessError),
}

impl From<AccessError> for ActionError {
    fn from(err: AccessError) -> Self {
        Self::AccessError(err)
    }
}

pub async fn create_story<A>(
    db: &A,
    user: &VerifiedUser,
    title: String,
    content: Vec<api::ContentDetails>,
) -> Result<api::Story, ActionError>
where
    A: access::story::AccessStory,
{
    let story = db.create_story(user, title).await?;
    let content = db.create_content(story.id, content).await?;

    Ok(api::Story::new(story, content))
}

pub async fn get_story<A>(
    db: &A,
    user: &VerifiedUser,
    story_uuid: Uuid,
) -> Result<api::Story, ActionError>
where
    A: access::story::AccessStory,
{
    let story = db.get_story_by_uuid(user, story_uuid).await?;
    let content = db.get_story_content(story.id).await?;

    Ok(api::Story::new(story, content))
}

pub async fn update_story<A>(
    db: &A,
    user: &VerifiedUser,
    mut story: model::Story,
    title: Option<String>,
    deleted: Option<bool>,
) -> Result<(), AppError>
where
    A: access::story::AccessStory,
{
    if let Some(title) = title.clone() {
        story.title = title;
    }

    if let Some(delete) = deleted.clone() {
        story.deleted = delete;
    }

    db.update_story(user, story).await?;

    Ok(())
}

pub async fn delete_story<A>(db: &A, user: &VerifiedUser, story_uuid: Uuid) -> Result<(), AppError>
where
    A: access::story::AccessStory,
{
    let mut story = db.get_story_by_uuid(user, story_uuid).await?;
    story.deleted = true;

    db.update_story(user, story).await?;

    Ok(())
}

pub struct ContentUpdate {
    pub uuid: Uuid,
    pub content: api::ContentDetails,
}

pub async fn update_content<A>(
    db: &A,
    story: &model::Story,
    updates: Vec<ContentUpdate>,
) -> Result<(), AppError>
where
    A: access::story::AccessStory,
{
    if story.deleted {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "Story has been deleted.".into(),
        ));
    }

    for u in updates {
        let content_id = db.get_content_by_uuid(u.uuid).await?.id;
        db.update_content(content_id, u.content).await?;
    }

    Ok(())
}

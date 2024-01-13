use async_trait::async_trait;
use uuid::Uuid;

use crate::{api, auth::VerifiedUser, model};

use super::schema;
use super::AccessError;

#[async_trait]
pub trait AccessStory {
    async fn create_story(
        &self,
        user: &VerifiedUser,
        title: String,
    ) -> Result<model::Story, AccessError>;
    async fn create_content(
        &self,
        story_id: u32,
        content: Vec<api::ContentDetails>,
    ) -> Result<Vec<model::Content>, AccessError>;
    async fn get_story_by_uuid(
        &self,
        user: &VerifiedUser,
        story_uuid: Uuid,
    ) -> Result<model::Story, AccessError>;
    async fn get_content_by_uuid(&self, content_uuid: Uuid) -> Result<model::Content, AccessError>;
    async fn get_story_content(&self, story_id: u32) -> Result<Vec<model::Content>, AccessError>;
    async fn update_story(
        &self,
        user: &VerifiedUser,
        story_updates: model::Story,
    ) -> Result<model::Story, AccessError>;
    async fn update_content(
        &self,
        content_id: u32,
        content_updates: api::ContentDetails,
    ) -> Result<model::Content, AccessError>;
}

#[async_trait]
impl AccessStory for super::MemoryDb {
    /// Creates a row in the `story` table.
    /// Must be used with `create_content` to populate the corresponding `content` rows.
    async fn create_story(
        &self,
        user: &VerifiedUser,
        title: String,
    ) -> Result<model::Story, AccessError> {
        let story_uuid = Uuid::new_v4();
        let story_id = sqlx::query!(
            "INSERT INTO stories (uuid, title, deleted, user_id) VALUES (?, ?, ?, ?)",
            story_uuid.to_string(),
            title,
            false,
            user.id()?
        )
        .execute(&self.inner)
        .await
        .map_err(AccessError::Sql)?
        .last_insert_id();

        let story = sqlx::query_as!(
            schema::Story,
            "SELECT * FROM stories WHERE id = ?",
            story_id
        )
        .fetch_one(&self.inner)
        .await
        .map_err(AccessError::Sql)?;

        Ok(story.try_into()?)
    }

    /// Creates a row(s) in the `content` table.
    /// Musted be used with `create_story` to first populate the corresponding `story` row.
    async fn create_content(
        &self,
        story_id: u32,
        content: Vec<api::ContentDetails>,
    ) -> Result<Vec<model::Content>, AccessError> {
        let mut db_content = Vec::new();
        for c in content {
            let content_uuid = Uuid::new_v4();
            let kind = c.kind();
            let details = c.details()?;

            let content_id = sqlx::query!(
                "INSERT INTO content (uuid, kind, details, story_id) VALUES (?, ?, ?, ?)",
                content_uuid.to_string(),
                kind,
                details,
                story_id
            )
            .execute(&self.inner)
            .await
            .map_err(AccessError::Sql)?
            .last_insert_id();

            let c = sqlx::query_as!(
                schema::Content,
                "SELECT * FROM content WHERE id = ?",
                content_id
            )
            .fetch_one(&self.inner)
            .await
            .map_err(AccessError::Sql)?
            .try_into()?;

            db_content.push(c);
        }
        Ok(db_content)
    }

    /// Returns one story that matches the provided story_uuid.
    /// For the story's content, see `get_story_content`
    async fn get_story_by_uuid(
        &self,
        user: &VerifiedUser,
        story_uuid: Uuid,
    ) -> Result<model::Story, AccessError> {
        let story = sqlx::query_as!(
            schema::Story,
            "SELECT * FROM stories WHERE user_id = ? AND uuid = ?",
            user.id()?,
            story_uuid.to_string()
        )
        .fetch_one(&self.inner)
        .await?
        .try_into()?;

        Ok(story)
    }

    async fn get_content_by_uuid(&self, content_uuid: Uuid) -> Result<model::Content, AccessError> {
        let content = sqlx::query_as!(
            schema::Content,
            "SELECT * FROM content WHERE uuid = ?",
            content_uuid.to_string()
        )
        .fetch_one(&self.inner)
        .await?
        .try_into()?;

        Ok(content)
    }

    /// Returns the specified story's content.
    async fn get_story_content(&self, story_id: u32) -> Result<Vec<model::Content>, AccessError> {
        let rows = sqlx::query_as!(
            schema::Content,
            "SELECT * FROM content WHERE story_id = ?",
            story_id
        )
        .fetch_all(&self.inner)
        .await?;

        let mut content = Vec::new();
        for c in rows.into_iter() {
            content.push(c.try_into()?);
        }

        Ok(content)
    }

    /// References the provided story [story_updates] to determine what updates to the row should be made.
    /// Only the row's `title` and `deleted` columns will be updated, if changed.
    async fn update_story(
        &self,
        user: &VerifiedUser,
        story_updates: model::Story,
    ) -> Result<model::Story, AccessError> {
        let story_id = sqlx::query!(
            "UPDATE stories SET title = ?, deleted = ? WHERE id = ? AND user_id = ?",
            story_updates.title,
            story_updates.deleted,
            story_updates.id,
            user.id()?
        )
        .execute(&self.inner)
        .await?
        .last_insert_id();

        let story = sqlx::query_as!(
            schema::Story,
            "SELECT * FROM stories WHERE id = ?",
            story_id
        )
        .fetch_one(&self.inner)
        .await?
        .try_into()?;

        Ok(story)
    }

    /// References the provided content [content_updates] to determine what updates to the row should be made.
    /// Only `kind` and `details` will be updated, if changed.
    async fn update_content(
        &self,
        content_id: u32,
        content_updates: api::ContentDetails,
    ) -> Result<model::Content, AccessError> {
        let kind = content_updates.kind();
        let details = content_updates.details()?;
        let content_id = sqlx::query!(
            "UPDATE content SET kind = ?, details = ? WHERE id = ?",
            kind,
            details,
            content_id
        )
        .execute(&self.inner)
        .await?
        .last_insert_id();

        let content = sqlx::query_as!(
            schema::Content,
            "SELECT * FROM content WHERE id = ?",
            content_id
        )
        .fetch_one(&self.inner)
        .await?
        .try_into()?;

        Ok(content)
    }
}

//! A slightly looser type structure that supports the types stored in the database.
//! These should be converted to their model counterparts.

use std::str::FromStr;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model;

#[derive(Debug)]
pub enum SchemaError {
    ParseUuid,
    ParseDeleted,
    ParseJson,
}

impl From<uuid::Error> for SchemaError {
    fn from(err: uuid::Error) -> Self {
        println!("{:?}", err);
        Self::ParseUuid
    }
}

impl From<serde_json::Error> for SchemaError {
    fn from(err: serde_json::Error) -> Self {
        println!("{:?}", err);
        Self::ParseJson
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub uuid: String,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<User> for model::User {
    type Error = SchemaError;

    fn try_from(u: User) -> Result<Self, Self::Error> {
        Ok(model::User {
            id: u.id,
            uuid: Uuid::from_str(&u.uuid)?,
            name: u.name,
            created_at: u.created_at,
            updated_at: u.updated_at,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Story {
    pub id: u32,
    pub user_id: u32,
    pub uuid: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted: i8,
}

impl TryFrom<Story> for model::Story {
    type Error = SchemaError;

    fn try_from(s: Story) -> Result<Self, Self::Error> {
        Ok(model::Story {
            id: s.id,
            user_id: s.user_id,
            uuid: Uuid::from_str(&s.uuid)?,
            title: s.title,
            created_at: s.created_at,
            updated_at: s.updated_at,
            deleted: match s.deleted {
                0 => false,
                1 => true,
                _ => return Err(SchemaError::ParseDeleted),
            },
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Content {
    pub id: u32,
    pub story_id: u32,
    pub uuid: String,
    pub kind: String,
    pub details: sqlx::types::JsonValue,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<Content> for model::Content {
    type Error = SchemaError;

    fn try_from(c: Content) -> Result<Self, Self::Error> {
        Ok(model::Content {
            id: c.id,
            story_id: c.story_id,
            uuid: Uuid::from_str(&c.uuid)?,
            kind: c.kind,
            details: serde_json::from_value(c.details)?,
            created_at: c.created_at,
            updated_at: c.updated_at,
        })
    }
}

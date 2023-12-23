use sqlx::MySqlPool;

use crate::api;

mod schema;
pub mod story;
pub mod user;

pub enum AccessError {
    Sql(sqlx::Error),
    Api(api::ApiError),
    Schema(schema::SchemaError),
}

impl From<schema::SchemaError> for AccessError {
    fn from(err: schema::SchemaError) -> Self {
        Self::Schema(err)
    }
}

impl From<sqlx::Error> for AccessError {
    fn from(err: sqlx::Error) -> Self {
        Self::Sql(err)
    }
}

impl From<api::ApiError> for AccessError {
    fn from(err: api::ApiError) -> Self {
        Self::Api(err)
    }
}

#[derive(Clone)]
pub struct MemoryDb {
    inner: MySqlPool,
}

impl MemoryDb {
    pub fn new(inner: MySqlPool) -> Self {
        Self { inner }
    }
}

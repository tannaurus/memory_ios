use anyhow::Context;
use axum::http::StatusCode;
use serde::{de::DeserializeOwned, Serialize};
use std::{
    fmt::Display,
    fs::{self, File},
    io::{BufReader, BufWriter},
};

use crate::AppError;

#[derive(Debug, Clone)]
pub enum DbEntity {
    Prompts,
    Stories,
    Content,
    Users,
}

impl Display for DbEntity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

/// Returns deserialized T, if found.
/// Returns [AppError] if file does not exist.
pub fn select_with_uuid<T>(kind: DbEntity, uuid: &str) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    let path = format!("db/{}/{}.json", kind, uuid);
    let file = File::open(&path).context(format!("Failed to open {}", path))?;
    let reader = BufReader::new(file);
    Ok(serde_json::from_reader(reader).context(format!("Failed to deserialize {}", path))?)
}

/// Selects all that match given id. The provided `id_column` is expected to exist in the struct `T`.
/// e.g: `id_column` could equal "story_id". "story_id" would be expected to be within T.
/// If "id_column" is not within T, this will return an empty vec.
pub fn select_all_by_id_column<T>(
    kind: DbEntity,
    id: usize,
    id_column: &str,
) -> Result<Vec<T>, AppError>
where
    T: DeserializeOwned,
{
    let path = format!("db/{}", kind);
    let dir = fs::read_dir(path).map_err(|_| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    let mut matches = Vec::new();
    for entry in dir {
        let entry = entry.map_err(|_| {
            AppError(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        })?;

        let path_uuid = {
            let path = entry.path().to_str().unwrap().to_owned();
            let start_filter_str = format!("db/{}/", kind);
            let path = path.trim_start_matches(&start_filter_str);
            path.trim_end_matches(".json").to_owned()
        };

        println!("{}", path_uuid);

        let found: serde_json::Value = select_with_uuid(kind.clone(), &path_uuid)?;
        if let serde_json::Value::Number(matched_id) = found[id_column].clone() {
            println!("matched id{}", matched_id);
            if matched_id.to_string() != id.to_string() {
                continue;
            }
            let matched: T = serde_json::from_value(found).map_err(|_| {
                AppError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            })?;

            matches.push(matched);
        }
    }

    Ok(matches)
}

/// Deletes the existing file, then writes it again.
pub fn update<T>(kind: DbEntity, uuid: &str, data: &T) -> Result<(), AppError>
where
    T: ?Sized + Serialize,
{
    let path = format!("db/{}/{}.json", kind, uuid);
    fs::remove_file(path).map_err(|_| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    write_db(kind, uuid, data)
}

/// Writes T into a new json file.
/// Returns [AppError] if something goes wrong.
pub fn write_db<T>(kind: DbEntity, uuid: &str, data: &T) -> Result<(), AppError>
where
    T: ?Sized + Serialize,
{
    let path = format!("db/{}/{}.json", kind, uuid);
    let file = File::create(&path).context(format!("Failed to create file {}", path))?;
    let writer = BufWriter::new(file);
    Ok(serde_json::to_writer(writer, data).context(format!("Failed to write {}", path))?)
}

/// Counts the length of items in the given [DbEntity] directory.
pub fn generate_entity_id(kind: DbEntity) -> Result<usize, AppError> {
    let path = format!("db/{}", kind);
    let dir = fs::read_dir(path).map_err(|_| {
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".to_string(),
        )
    })?;

    let mut count = 1;
    for _ in dir {
        count += 1;
    }

    Ok(count)
}

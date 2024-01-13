use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};

use crate::{access::user::AccessUser, api, auth, AppContext, AppError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
}

/// Creates a user and updates the [AppContext] to contain [auth::VerifiedUser].
pub async fn create_user(
    mut ctx: State<AppContext>,
    request: Json<CreateUserRequest>,
) -> Result<Json<api::User>, AppError> {
    if ctx.auth.authenticated().is_ok() {
        return Err(AppError(
            StatusCode::BAD_REQUEST,
            "Already signed in.".into(),
        ));
    }
    let user = ctx.db.create_user(request.name.clone()).await?;

    ctx.auth.set_user(auth::VerifiedUser::new(user.clone()));

    Ok(Json(user.into()))
}

/// Returns a verified user's profile information
pub async fn get_verified_user(ctx: State<AppContext>) -> Result<Json<api::User>, AppError> {
    let user = ctx.auth.authenticated()?;

    let user = ctx.db.get_user(user).await?;

    Ok(Json(user.into()))
}

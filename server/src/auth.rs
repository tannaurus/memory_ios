use std::sync::{Arc, Mutex, MutexGuard};

use axum::http::StatusCode;
use once_cell::sync::Lazy;
use uuid::Uuid;
use webauthn_rs::{prelude::Url, Webauthn, WebauthnBuilder};

use crate::{model, AppError, AppResult};

static AUTH_APP_ID: &'static str = "memory.io";
static AUTH_APP_ORIGIN: Lazy<Url> = Lazy::new(|| Url::parse("https://memory.io").unwrap());

#[derive(Clone)]
pub struct AuthState {
    pub webauthn: Arc<Webauthn>,
    verified_user: Option<VerifiedUser>,
}

impl AuthState {
    /// Determines if a user is signed in.
    /// If not, returns AppError(StatusCode::UNAUTHORIZED, "Login required")
    pub fn authenticated(&self) -> AppResult<&VerifiedUser> {
        self.verified_user
            .as_ref()
            .ok_or_else(|| AppError(StatusCode::UNAUTHORIZED, "Login required".into()))
    }

    /// Sets the verified user.
    pub fn set_user(&mut self, user: VerifiedUser) {
        self.verified_user = Some(user)
    }
}

impl AuthState {
    /// Creates a new Webauthn client. Will panic if configuration is wrong.
    pub fn new() -> Self {
        let builder = WebauthnBuilder::new(AUTH_APP_ID, &AUTH_APP_ORIGIN).unwrap();
        let builder = builder.rp_name("Memory");

        let webauthn = builder.build().unwrap();

        Self {
            webauthn: Arc::new(webauthn),
            verified_user: None,
        }
    }
}

#[derive(Clone)]
pub struct VerifiedUser {
    inner: Arc<Mutex<model::User>>,
}

impl VerifiedUser {
    pub fn new(user: model::User) -> Self {
        Self {
            inner: Arc::new(Mutex::new(user)),
        }
    }

    fn user_lock_safe(&self) -> Result<MutexGuard<model::User>, AppError> {
        let verified_user = match self.inner.lock() {
            Ok(user_lock) => user_lock,
            Err(_) => {
                return Err(AppError(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "VerifiedUser was poisoned.".into(),
                ))
            }
        };

        Ok(verified_user)
    }

    /// Acquires a lock on the inner user and clones the user's UUID to allow the lock
    /// to drop as soon as possible.
    ///
    /// Returns an owned UUID.
    /// Returns an AppError if the Mutex was poisoned.
    pub fn uuid(&self) -> Result<Uuid, AppError> {
        let verified_user = self.user_lock_safe()?;

        Ok(verified_user.uuid.clone())
    }

    /// Acquires a lock on the inner user and clones the user's ID to allow the lock
    /// to drop as soon as possible.
    ///
    /// Returns an owned UUID.
    /// Returns an AppError if the Mutex was poisoned.
    pub fn id(&self) -> Result<u32, AppError> {
        let verified_user = self.user_lock_safe()?;

        Ok(verified_user.id.clone())
    }
}

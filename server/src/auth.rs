use crate::model;

#[derive(Clone)]
pub struct AuthState {
    pub user: model::User,
}

use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use clap::Parser;
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

mod access;
mod action;
mod api;
mod auth;
mod handlers;
mod model;

use access::MemoryDb;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to accept requests from
    #[arg(short, long, env = "LISTENER", default_value = "127.0.0.1:3000")]
    listener: SocketAddr,

    #[arg(env = "MYSQL_DATABASE", default_value = "memory")]
    mysql_database: String,
    #[arg(env = "MYSQL_USER", default_value = "user")]
    mysql_user: String,
    #[arg(env = "MYSQL_PASSWORD", default_value = "root")]
    mysql_password: String,
    #[arg(env = "MYSQL_PORT", default_value = "3306")]
    mysql_port: String,
    #[arg(env = "MYSQL_HOST", default_value = "localhost:3306")]
    mysql_host: String,
}

#[derive(Debug)]
pub struct AppError(StatusCode, String);

pub type AppResult<T> = Result<T, AppError>;

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, error.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        (self.0, format!("{}", self.1)).into_response()
    }
}

impl From<access::AccessError> for AppError {
    fn from(err: access::AccessError) -> Self {
        println!("{:?}", err);
        AppError(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Internal server error".into(),
        )
    }
}

impl From<action::ActionError> for AppError {
    fn from(err: action::ActionError) -> Self {
        match err {
            action::ActionError::AccessError(err) => err.into(),
        }
    }
}
#[derive(Clone)]
pub struct AppContext {
    pub db: MemoryDb,
    pub auth: auth::AuthState,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mysql_url = format!(
        "mysql://{}:{}@{}/{}",
        args.mysql_user, args.mysql_password, args.mysql_host, args.mysql_database
    );

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&mysql_url)
        .await
        .unwrap();

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let auth_state = auth::AuthState::new();

    let context = AppContext {
        db: MemoryDb::new(pool),
        auth: auth_state,
    };

    let app = Router::new()
        .route("/prompts", get(handlers::get_prompts))
        .route("/user", post(handlers::user::create_user))
        .route("/user", get(handlers::user::get_verified_user))
        .route(
            "/stories/:story_uuid",
            get(handlers::story::handle_get_story),
        )
        .route("/story", post(handlers::story::handle_create_story))
        .route(
            "/story/:story_uuid",
            delete(handlers::story::handle_delete_story),
        )
        .route(
            "/story/:story_uuid",
            put(handlers::story::handle_update_story),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(context);

    println!("Listening on {} 🚀", &args.listener);
    axum::Server::bind(&args.listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

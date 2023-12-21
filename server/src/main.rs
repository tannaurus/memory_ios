use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Router,
};
use clap::Parser;
use serde_json::json;
use sqlx::mysql::MySqlPoolOptions;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

mod access;
mod action;
mod api;
mod auth;
mod handlers;
mod model;

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

    let mocked_user: model::User = {
        let user_json = json!({
            "id": 1,
            "uuid": "6c81e345-1ab3-463b-8aa2-916da81c1d0c",
            "name": "Tanner Gill"
        });
        serde_json::from_value(user_json).unwrap()
    };
    let auth_state = auth::AuthState { user: mocked_user };

    let app = Router::new()
        .route("/prompts", get(handlers::get_prompts))
        .route("/user", get(handlers::get_user))
        .route(
            "/stories/:story_uuid",
            get(handlers::stories::handle_get_story),
        )
        .route("/story", post(handlers::stories::handle_create_story))
        .route(
            "/story/:story_uuid",
            delete(handlers::stories::handle_delete_story),
        )
        .route(
            "/story/:story_uuid",
            put(handlers::stories::handle_update_story),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(auth_state);

    println!("Listening on {} 🚀", &args.listener);

    axum::Server::bind(&args.listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

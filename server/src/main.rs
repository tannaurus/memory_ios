use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;
use tower_http::trace::TraceLayer;

mod handlers;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to accept requests from
    #[arg(short, long, env = "LISTENER", default_value = "127.0.0.1:3000")]
    listener: SocketAddr,
}

struct AppError(StatusCode, anyhow::Error);

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

    let app = Router::new()
        .route("/prompts", get(handlers::get_prompts))
        .route("/user", get(handlers::get_user))
        .layer(TraceLayer::new_for_http());

    println!("Listening on {} 🚀", &args.listener);

    axum::Server::bind(&args.listener)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

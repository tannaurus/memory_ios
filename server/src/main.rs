use axum::{routing::get, Router};
use clap::Parser;
use std::net::SocketAddr;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Address to accept requests from
    #[arg(short, long, default_value = "127.0.0.1:3000")]
    addr: SocketAddr,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }));

    axum::Server::bind(&args.addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

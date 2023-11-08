use axum::{
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(handler))
        .route("/todos", get(get_todos))
        .route("/test", get(test));

    async fn handler() -> &'static str {
        "Welcome to the Rust Axum REST API!"
    }

    async fn get_todos() -> &'static str {
        "List of todos will go here"
    }

    async fn test() -> &'static str {
        "this is a test route"
    }

    //let addr = SocketAddr::from(([192, 168, X, X], 3000));//for local Router testing
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    
    // Start the Axum server
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

use axum::{routing::get, Router};
use std::net::SocketAddr;
use ticket_toy_server::config;
use ticket_toy_server::service;

#[tokio::main]
async fn main() {
    let app_state = config::context::load().await.unwrap();

    let app = Router::new()
        .route("/health", get("OK"))
        .nest("/", service::movie_routes())
        .nest("/user", service::user_routes())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

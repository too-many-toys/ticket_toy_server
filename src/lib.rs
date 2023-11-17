use std::net::SocketAddr;

use anyhow::Context;
use axum::{
    http::header::{AUTHORIZATION, CONTENT_TYPE},
    http::HeaderValue,
    routing::get,
    Router,
};
use tower_http::cors::{Any, CorsLayer};

pub mod auth;
pub mod config;
pub mod errors;
pub mod model;
pub mod request;
pub mod service;

pub async fn start_server() -> Result<(), anyhow::Error> {
    let app_state = config::context::load().await.unwrap();

    let app = Router::new()
        .route("/health", get("OK"))
        .nest("/movie", service::movie_routes())
        .nest("/user", service::user_routes())
        .layer(
            CorsLayer::new()
                .allow_origin([
                    "http://localhost:7357".parse::<HeaderValue>().unwrap(),
                    "https://totd.xyz".parse::<HeaderValue>().unwrap(),
                ])
                .allow_methods(Any)
                .allow_headers(vec![CONTENT_TYPE, AUTHORIZATION]),
        )
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal())
        .await
        .context("HTTP server error")?;

    Ok(())
}

async fn shutdown_signal() {
    tokio::signal::ctrl_c()
        .await
        .expect("Expect shutdown signal handler");
    println!("Server Down")
}

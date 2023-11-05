use std::net::SocketAddr;

use anyhow::Context;
use axum::{routing::get, Router};

pub mod auth;
pub mod config;
pub mod errors;
pub mod model;
pub mod service;

pub async fn start_server() -> Result<(), anyhow::Error> {
    let app_state = config::context::load().await.unwrap();

    let app = Router::new()
        .route("/health", get("OK"))
        .nest("/", service::movie_routes())
        .nest("/user", service::user_routes())
        .nest("/oauth", service::oauth_routers())
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

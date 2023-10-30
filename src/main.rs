use axum::{routing::get, Router};
use dotenv;
use std::net::SocketAddr;
use ticket_toy_server::config::{AppState, MovieState};
use ticket_toy_server::service;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        movie_state: MovieState {
            api_key: std::env::var("MOVIE_API_KEY").unwrap(),
        },
    };

    let app = Router::new()
        .route("/health", get("OK"))
        .nest("/", service::movie_routes())
        .with_state(app_state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

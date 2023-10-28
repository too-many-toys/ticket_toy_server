use axum::{routing::get, Router};
use dotenv;
use std::{net::SocketAddr, sync::Arc};

mod errors;
mod movie;
mod state;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let movie_state = Arc::new(state::MovieState {
        movie_api_key: std::env::var("MOVIE_API_KEY").unwrap(),
    });

    let movie_routes = movie::movie_routes(movie_state);

    let app = Router::new()
        .route("/health", get("OK"))
        .nest("/", movie_routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

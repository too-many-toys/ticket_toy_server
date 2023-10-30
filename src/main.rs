use axum::{routing::get, Router};
use dotenv;
use std::net::SocketAddr;
use ticket_toy_server::config::{AppState, MovieState, UserState};
use ticket_toy_server::service;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let app_state = AppState {
        movie_state: MovieState {
            api_key: std::env::var("MOVIE_API_KEY").unwrap(),
        },
        user_state: UserState {
            kakao_api_key: std::env::var("KAKAO_API_KEY").unwrap(),
            kakao_redirect_url: std::env::var("KAKAO_REDIRECT_URL").unwrap(),
        },
        db_state: ticket_toy_server::config::DBState {
            db_url: std::env::var("DB_URL").unwrap(),
            db_name: std::env::var("DB_NAME").unwrap(),

            client: mongodb::Client::with_uri_str(&std::env::var("DB_URL").unwrap())
                .await
                .unwrap(),
        },
    };

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

use super::{AppState, DBState, MovieState, UserState};
use dotenv;
use mongodb::{options::ClientOptions, Client, Database};

pub async fn load() -> Result<AppState, mongodb::error::Error> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_name = std::env::var("DB_NAME").unwrap();
    let db_url = std::env::var("DB_URL").unwrap();

    let client = connect_db(&db_url, &db_name).await?;

    let app_state = AppState {
        movie_state: MovieState {
            api_key: std::env::var("MOVIE_API_KEY").unwrap(),
        },
        user_state: UserState {
            kakao_api_key: std::env::var("KAKAO_API_KEY").unwrap(),
            kakao_redirect_url: std::env::var("KAKAO_REDIRECT_URL").unwrap(),
        },
        db_state: DBState {
            db_url,
            db_name,
            client,
        },
    };

    Ok(app_state)
}

pub async fn connect_db(
    db_url: &String,
    db_name: &String,
) -> Result<Database, mongodb::error::Error> {
    let client_options = ClientOptions::parse(db_url).await?;

    let client = Client::with_options(client_options)?;

    let db = client.database(&db_name);

    tracing::info!("Connected to database: {}", db_name);

    Ok(db)
}

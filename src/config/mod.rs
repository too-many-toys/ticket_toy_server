use axum::extract::FromRef;

pub mod context;

#[derive(Clone)]
pub struct AppState {
    pub movie_state: MovieState,
    pub user_state: UserState,
    pub db_state: DBState,
}

#[derive(Clone)]
pub struct MovieState {
    pub api_key: String,
}

impl FromRef<AppState> for MovieState {
    fn from_ref(app_state: &AppState) -> MovieState {
        app_state.movie_state.clone()
    }
}

#[derive(Clone)]
pub struct UserState {
    pub kakao_api_key: String,
    pub kakao_redirect_url: String,
}

impl FromRef<AppState> for UserState {
    fn from_ref(app_state: &AppState) -> UserState {
        app_state.user_state.clone()
    }
}

#[derive(Clone)]
pub struct DBState {
    pub db_url: String,
    pub db_name: String,

    pub client: mongodb::Client,
}

impl FromRef<AppState> for DBState {
    fn from_ref(app_state: &AppState) -> DBState {
        app_state.db_state.clone()
    }
}

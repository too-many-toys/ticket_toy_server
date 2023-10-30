use axum::extract::FromRef;

#[derive(Clone)]
pub struct AppState {
    pub movie_state: MovieState,
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

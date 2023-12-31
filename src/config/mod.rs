use axum::extract::FromRef;
use mongodb::Collection;

use crate::model::{movie::Movie, my_collection::MyCollection, user::User};

pub mod context;

#[derive(Clone)]
pub struct AppState {
    pub movie_state: MovieState,
    pub user_state: UserState,
    pub auth_state: AuthState,
    pub my_collection_state: MyCollectionState,
}

#[derive(Clone)]
pub struct MovieState {
    pub api_key: String,
    pub collection: Collection<Movie>,
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
    pub collection: Collection<User>,
}

impl FromRef<AppState> for UserState {
    fn from_ref(app_state: &AppState) -> UserState {
        app_state.user_state.clone()
    }
}

#[derive(Clone)]
pub struct AuthState {
    pub jwt_secret: String,
}

impl FromRef<AppState> for AuthState {
    fn from_ref(app_state: &AppState) -> AuthState {
        app_state.auth_state.clone()
    }
}

#[derive(Clone)]
pub struct MyCollectionState {
    pub collection: Collection<MyCollection>,
}

impl FromRef<AppState> for MyCollectionState {
    fn from_ref(app_state: &AppState) -> MyCollectionState {
        app_state.my_collection_state.clone()
    }
}

use crate::model::{
    me_too::MeToo,
    movie::Movie,
    my_collection::{collection_schema, MyCollection},
    user::{user_schema, User},
};

use super::{AppState, AuthState, MovieState, MyCollectionState, UserState};
use dotenv;
use mongodb::{
    bson::doc,
    options::{
        ClientOptions, ClusteredIndex, CreateCollectionOptions, ValidationAction, ValidationLevel,
    },
    Client, Collection, Database, IndexModel,
};

pub async fn load() -> Result<AppState, mongodb::error::Error> {
    // TODO: for not production
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db_name = std::env::var("DB_NAME").unwrap();
    let db_url = std::env::var("DB_URL").unwrap();

    let client = connect_db(&db_url, &db_name).await?;

    create_user_collection(&client).await;
    create_my_collection_collection(&client).await;
    create_me_too_collection(&client).await;
    create_my_collection_index(client.collection::<MyCollection>("mycollections")).await;
    create_me_too_index(client.collection::<MeToo>("me_too")).await;

    let app_state = AppState {
        movie_state: MovieState {
            api_key: std::env::var("MOVIE_API_KEY").unwrap(),
            collection: client.collection::<Movie>("movies"),
        },
        user_state: UserState {
            kakao_api_key: std::env::var("KAKAO_API_KEY").unwrap(),
            kakao_redirect_url: std::env::var("KAKAO_REDIRECT_URL").unwrap(),
            collection: client.collection::<User>("users"),
        },
        auth_state: AuthState {
            jwt_secret: std::env::var("JWT_SECRET").unwrap(),
        },
        my_collection_state: MyCollectionState {
            collection: client.collection::<MyCollection>("mycollections"),
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

pub async fn create_user_collection(db: &Database) {
    let is_exists = db.list_collection_names(doc! {}).await.unwrap();
    for name in is_exists {
        if name == "users" {
            return;
        }
    }

    let clustered_index = ClusteredIndex::default();
    let validation_opts = CreateCollectionOptions::builder()
        // .validator(user_schema())
        // .validation_action(Some(ValidationAction::Error))
        // .validation_level(Some(ValidationLevel::Strict))
        .clustered_index(clustered_index)
        .build();

    match db.create_collection("users", validation_opts).await {
        Ok(_) => {
            tracing::info!("Created collection: users");
        }
        Err(e) => {
            tracing::error!("Failed to create collection: users");
            tracing::error!("Error: {}", e);
        }
    }
}

pub async fn create_my_collection_collection(db: &Database) {
    let is_exists = db.list_collection_names(doc! {}).await.unwrap();
    for name in is_exists {
        if name == "mycollections" {
            return;
        }
    }

    let clustered_index = ClusteredIndex::default();
    let validation_opts = CreateCollectionOptions::builder()
        // .validator(collection_schema())
        // .validation_action(Some(ValidationAction::Error))
        // .validation_level(Some(ValidationLevel::Strict))
        .clustered_index(clustered_index)
        .build();

    match db.create_collection("mycollections", validation_opts).await {
        Ok(_) => {
            tracing::info!("Created collection: mycollections");
        }
        Err(e) => {
            tracing::error!("Failed to create collection: mycollections");
            tracing::error!("Error: {}", e);
        }
    }
}

pub async fn create_my_collection_index(col: Collection<MyCollection>) {
    // 인덱스 만들 때 먼저 지우기 옵션
    // let drop_opt = DropIndexesOptions::builder().build();
    // col.drop_indexes(drop_opt).await;

    let author_index = IndexModel::builder().keys(doc! {"author_id": 1}).build();
    let is_post_index = IndexModel::builder().keys(doc! {"is_post": 1}).build();

    match col.create_index(author_index, None).await {
        Ok(_) => {
            tracing::info!("Created index: mycollections.author_id");
        }
        Err(e) => {
            tracing::error!("Failed to create index: mycollections.author_id");
            tracing::error!("Error: {}", e);
        }
    }

    match col.create_index(is_post_index, None).await {
        Ok(_) => {
            tracing::info!("Created index: collections.is_post");
        }
        Err(e) => {
            tracing::error!("Failed to create index: collections.is_post");
            tracing::error!("Error: {}", e);
        }
    }
}

pub async fn create_me_too_collection(db: &Database) {
    let is_exists = db.list_collection_names(doc! {}).await.unwrap();
    for name in is_exists {
        if name == "me_too" {
            return;
        }
    }

    let clustered_index = ClusteredIndex::default();
    let validation_opts = CreateCollectionOptions::builder()
        // .validator(doc! {})
        // .validation_action(Some(ValidationAction::Error))
        // .validation_level(Some(ValidationLevel::Strict))
        .clustered_index(clustered_index)
        .build();

    match db.create_collection("me_too", validation_opts).await {
        Ok(_) => {
            tracing::info!("Created collection: me_too");
        }
        Err(e) => {
            tracing::error!("Failed to create collection: me_too");
            tracing::error!("Error: {}", e);
        }
    }
}

pub async fn create_me_too_index(col: Collection<MeToo>) {
    let col_index = IndexModel::builder()
        .keys(doc! {"my_collection_id": 1})
        .build();

    match col.create_index(col_index, None).await {
        Ok(_) => {
            tracing::info!("Created index: me_too.my_collection_id");
        }
        Err(e) => {
            tracing::error!("Failed to create index: me_too.my_collection_id");
            tracing::error!("Error: {}", e);
        }
    }
}

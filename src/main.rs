mod structs;
mod idgenerator;
mod repository;
mod service;

use crate::repository::Repository;
use crate::repository::PostFilters;
use crate::repository::UserFilters;
use crate::repository::UserMetaFilters;
use crate::repository::PostmetaFilters;

use structs::user::User;
use structs::post::Post;
use structs::postmeta::PostMeta;
use structs::usermeta::UserMeta;

use repository::user_repository::SqliteUserRepository;
use repository::post_repository::SqlitePostRepository;

use service::post_service::PostService;
use service::user_service::UserService;
use service::postmeta_service::PostMetaService;
use service::usermeta_service::UserMetaService;

use std::sync::Arc;
use tokio::sync::Mutex;
use http::Method;
use tower_http::cors::CorsLayer;
use http::HeaderValue;
use axum::Router;
use axum::routing::{get, post};
use axum::Json;
use axum::extract::State;

use serde_json::{Value, json};

mod database;

use database::SqliteDatabase;

use crate::{
    repository::post_repository::PostRepository,
    database::{Database},
};

//type RustPressState<'a> = AppState<SqlitePostRepository<'a>, SqliteUserRepository<'a>, UserMetaRepository<'a>, SqlitePostmetaRepository<'a>>;

struct AppState {
    database: Mutex<SqliteDatabase>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let db_url: &str = "sqlite://blog.db";

    let database: SqliteDatabase = SqliteDatabase::new(db_url);
    
    let shared_state = Arc::new(AppState {
        database: Mutex::new(database),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080/".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .route("/posts", get(get_posts))
        .with_state(shared_state)
        .layer(cors);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

//type PostRepository = Repository<Post, PostFilters>;

//#[axum::debug_handler]
async fn add_post(State(state): State<Arc<AppState>>, Json(payload): Json<Post>) -> Json<Value> {
   
    let database = state.database.lock().await;
    let post_repository = database.post_repository();
    let post_service = PostService::new(post_repository);
    let post_id = 0;
    
    Json(json!(
        post_id
    ))

}

async fn get_posts(State(state): State<Arc<AppState>>) -> Json<Value> {

    let database = state.database.lock().await;
    let post_repository = database.post_repository();
    let post_service = PostService::new(post_repository);

    let posts = post_service.get_posts();

    Json(json!(
        posts
    ))

}

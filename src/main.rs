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

//use repository::postmeta_repository::SqlitePostmetaRepository;
//use repository::usermeta_repository::UserMetaRepository;
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
use axum::routing::get;
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

struct AppState<
    PostRepository: Repository<Post, PostFilters>,
    UserRepository: Repository<User, UserFilters>,
    UserMetaRepository: Repository<UserMeta, UserMetaFilters>, 
    PostMetaRepository: Repository<PostMeta, PostmetaFilters>
> {
    post_service: Mutex<PostService<PostRepository>>,
    user_service: Mutex<UserService<UserRepository>>,
    postmeta_service: Mutex<PostMetaService<PostMetaRepository>>,
    usermeta_service: Mutex<UserMetaService<UserMetaRepository>>,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let db_url: &str = "sqlite://blog.db";

    let database: SqliteDatabase = SqliteDatabase::new(db_url);

    let post_repository = database.post_repository();
    let user_repository = database.user_repository();
    let usermeta_repository = database.usermeta_repository();
    let postmeta_repository = database.postmeta_repository();

    let post_service = PostService::new(post_repository);
    let user_service = UserService::new(user_repository);
    let postmeta_service = PostMetaService::new(postmeta_repository);
    let usermeta_service = UserMetaService::new(usermeta_repository);

    let shared_state = Arc::new(AppState {
        post_service: Mutex::new(post_service),
        user_service: Mutex::new(user_service),
        postmeta_service: Mutex::new(postmeta_service),
        usermeta_service: Mutex::new(usermeta_service),
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

async fn get_posts<
        PostRepository: Repository<Post, PostFilters>,
        UserRepository: Repository<User, UserFilters>,
        PostmetaRepository: Repository<PostMeta, PostmetaFilters>,
        UserMetaRepository: Repository<UserMeta, UserMetaFilters>,
    >(State(state): State<Arc<AppState<PostRepository, UserRepository, UserMetaRepository, PostmetaRepository>>>) -> Json<Value> {

    let post_service = state.post_service.lock().await;
    let posts = post_service.get_posts();

    Json(json!(
        posts
    ))

}

mod structs;
mod idgenerator;
mod repository;
mod service;
mod serviceprovider;
mod routes;
mod errors;

use crate::repository::Repository;
use crate::repository::PostFilters;
use crate::repository::UserFilters;
use crate::repository::UserMetaFilters;
use crate::repository::PostmetaFilters;

use http::StatusCode;

use structs::user::User;
use structs::post::Post;
use structs::postmeta::PostMeta;
use structs::usermeta::UserMeta;

use crate::serviceprovider::ServiceProvider;



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
use axum::routing::{get, post};
use axum::response::Json;
use axum::extract::State;

use serde_json::{Value, json};

mod database;

use database::SqliteDatabase;

use crate::{
    repository::post_repository::PostRepository,
    database::{Database},
};


#[derive(Clone)]
struct AppState {
    service_provider: Arc<Mutex<ServiceProvider>>,
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Sorry, that isn't available")
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {

    let db_url: &str = "sqlite://blog.db";

    let service_provider: ServiceProvider = {
        let database: SqliteDatabase = SqliteDatabase::new(db_url);
        ServiceProvider::new(database)
    }; 

    let shared_state = Arc::new(AppState {
        service_provider: Arc::new(Mutex::new(service_provider)),
    });

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:8080/".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    let app = Router::new()
        .nest("/posts", routes::posts::router())
        .nest("/users", routes::users::router())
        .nest("/postmeta", routes::post_meta::router())
        .nest("/usermeta", routes::user_meta::router())
        .with_state(shared_state)
        .layer(cors)
        .fallback(fallback);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}







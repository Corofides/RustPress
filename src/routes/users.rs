use axum::Json;
use crate::UserFilters;
use axum::extract::Path;
use axum::extract::State;
use axum::extract::Query;
use serde_json::Value;
use serde_json::json;
use async_std::sync::Arc;
use crate::UserMetaFilters;
use crate::AppState;
use crate::User;
use axum::Router;
use axum::routing::{
    get, post
};
use crate::errors::request_error::RequestError;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(add_user))
        .route("/{id}", get(get_user))
        .route("/{id}/meta", get(get_user_meta))
}

async fn add_user(State(state): State<Arc<AppState>>, Json(user): Json<User>) -> Result<Json<u32>, RequestError> {

    let service_provider = state.service_provider.lock().await;
    let user_service = service_provider.user_service();

    let result = user_service.create_user(user);

    match result {
        Ok(result) => {
            return Ok(Json(0));
        }
        Err(_) => {
            return Err(RequestError::CreationError)
        }
    }
}

async fn get_users(State(state): State<Arc<AppState>>, Query(filters): Query<UserFilters>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let user_service = service_provider.user_service();

    let users = user_service.get_users(filters);

    Json(json!(
        users
    ))
}

async fn get_user(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let user_service = service_provider.user_service();

    let user = user_service.get_user(id);

    Json(json!(
        user
    ))
}

async fn get_user_meta(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let user_meta_service = service_provider.user_meta_service();

    let filters = UserMetaFilters::new()
        .add_user(&id);

    let user_meta = user_meta_service.get_user_meta(filters);

    Json(json!(
        user_meta
    ))
}

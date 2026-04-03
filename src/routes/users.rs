use axum::Json;
use axum::extract::Path;
use axum::extract::State;
use serde_json::Value;
use serde_json::json;
use async_std::sync::Arc;
use crate::AppState;
use crate::User;
use axum::Router;
use axum::routing::{
    get, post
};

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_users))
        .route("/", post(add_user))
        .route("/{id}", get(get_user))
}

async fn add_user(State(state): State<Arc<AppState>>, Json(payload): Json<User>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let user_service = service_provider.user_service();

    let user_id = 0;

    Json(json!(
        user_id
    ))
}

async fn get_users(State(state): State<Arc<AppState>>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let user_service = service_provider.user_service();

    let users = user_service.get_users();

    Json(json!(
        users
    ))
}

async fn get_user(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let user_meta = service_provider.user_service();

    let user_meta = user_meta.get_user(id);

    Json(json!(
        user_meta
    ))
}

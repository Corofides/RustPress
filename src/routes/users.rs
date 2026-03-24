use axum::Json;
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
        .route("/users", get(get_users))
        .route("/users", post(add_user))
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

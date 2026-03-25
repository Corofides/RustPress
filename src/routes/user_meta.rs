use crate::Post;
use axum::Router;
use axum::Json;
use serde_json::Value;
use serde_json::json;
use axum::routing::{
    Route,
    MethodRouter,
    get, post,
};
use crate::State;
use crate::AppState;
use async_std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/usermeta", get(get_user_meta))
}

async fn get_user_meta(State(state): State<Arc<AppState>>) -> Json<Value> {
    
    let service_provider = state.service_provider.lock().await;
    let user_meta_service = service_provider.user_meta_service();

    let user_meta = user_meta_service.get_user_meta();

    Json(json!(
        user_meta
    ))

}

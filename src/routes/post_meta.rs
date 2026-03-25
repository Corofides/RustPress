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
        .route("/postmeta", get(get_post_meta))
}

async fn get_post_meta(State(state): State<Arc<AppState>>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let post_meta_service = service_provider.post_meta_service();

    let post_meta = post_meta_service.get_post_meta();

    Json(json!(
        post_meta
    ))

}

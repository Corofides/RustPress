use crate::Post;
use axum::Router;
use axum::Json;
use serde_json::Value;
use serde_json::json;
use axum::routing::{
    Route,
    get, post,
};
use crate::State;
use crate::AppState;
use async_std::sync::Arc;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_posts))
        .route("/", post(add_post))
}

async fn add_post(State(state): State<Arc<AppState>>, Json(payload): Json<Post>) -> Json<Value> {
  
    let service_provider = state.service_provider.lock().await;
    let post_service = service_provider.post_service();

    let post_id = 0;
    
    Json(json!(
        post_id
    ))

}

async fn get_posts(State(state): State<Arc<AppState>>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let post_service = service_provider.post_service();
    
    let posts = post_service.get_posts();

    Json(json!(
        posts
    ))

}


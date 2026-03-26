use crate::Post;
use axum::extract::Path;
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

pub fn get_routes() -> Vec<(String, String, MethodRouter<Arc<AppState>>)> {
    vec![
        ("/".to_string(), "GET".to_string(), get(get_posts)),
        ("/".to_string(), "POST".to_string(), post(add_post)),
        ("/{id}".to_string(), "GET".to_string(), get(get_post)),
    ]
}

pub fn router() -> Router<Arc<AppState>> {
    
    let routes = get_routes();

    let mut router = Router::new();

    for (path, method, operation) in routes {
        router = router.route(&path, operation);
    }

    return router;
}

async fn add_post(State(state): State<Arc<AppState>>, Json(payload): Json<Post>) -> Json<Value> {
  
    let service_provider = state.service_provider.lock().await;
    let post_service = service_provider.post_service();

    let post_id = 0;
    
    Json(json!(
        post_id
    ))

}

async fn get_post(State(state): State<Arc<AppState>>, Path(payload): Path<u32>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let post_service = service_provider.post_service();

    let post = post_service.get_post(payload);

    Json(json!(
        post
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


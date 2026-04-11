use crate::errors::request_error::RequestError;
use crate::Post;
use crate::PostFilters;
use crate::PostmetaFilters;
use axum::extract::{Path, Query};
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
use crate::PostMeta;
use async_std::sync::Arc;
use http::StatusCode;
use axum::response::{
    Response,
    IntoResponse,
};

pub fn get_routes() -> Vec<(String, String, MethodRouter<Arc<AppState>>)> {
    vec![
        ("/".to_string(), "GET".to_string(), get(get_posts)),
        ("/".to_string(), "POST".to_string(), post(add_post)),
        ("/{id}".to_string(), "GET".to_string(), get(get_post)),
        ("/{id}/meta".to_string(), "GET".to_string(), get(get_post_meta)),
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

async fn add_post(State(state): State<Arc<AppState>>, Json(post): Json<Post>) -> Result<Json<Value>, RequestError> {
  
    let service_provider = state.service_provider.lock().await;
    let post_service = service_provider.post_service();

    let result = post_service.add_post(post);

    match result {
        Ok(_) => {
            return Ok(Json(json!(
               0 
            )));
        }
        Err(_) => {
            return Err(RequestError::CreationError);
        }
    }         

    
}

#[axum::debug_handler]
async fn get_post(State(state): State<Arc<AppState>>, Path(payload): Path<u32>) -> Result<Json<Post>, RequestError> {

    let service_provider = state.service_provider.lock().await;

    let post_service = service_provider.post_service();

    let post = post_service.get_post(payload);

    let Some(post) = post else {
        return Err(RequestError::NotFound("Post".to_string()));
    };

    Ok(Json(
        post
    ))
}

async fn get_post_meta(State(state): State<Arc<AppState>>, Path(post_id): Path<u32>) -> Result<Json<Vec<PostMeta>>, RequestError> {

    let service_provider = state.service_provider.lock().await;

    let post_meta_service = service_provider.post_meta_service();

    let filters = PostmetaFilters::new().set_post(&post_id);

    let post_meta = post_meta_service.get_post_meta(filters);

    if post_meta.is_empty() {
        let post_service = service_provider.post_service();
        let exists = post_service.exists(&post_id);

        if !exists {
            return Err(RequestError::NotFound("Post".to_string()));
        }
    }

    Ok(Json(
        post_meta
    ))
}

#[axum::debug_handler]
async fn get_posts(State(state): State<Arc<AppState>>, Query(filters): Query<PostFilters>) -> Result<Json<Vec<Post>>, RequestError> {

    let service_provider = state.service_provider.lock().await;
    let post_service = service_provider.post_service();

    let posts = post_service.get_filtered_posts(filters);

    Ok(Json(
        posts
    ))

}


use crate::errors::request_error::RequestError;
use crate::Post;
use axum::Router;
use axum::Json;
use axum::extract::Query;
use crate::PostmetaFilters;
use serde_json::Value;
use serde_json::json;
use axum::extract::Path;
use axum::routing::{
    Route,
    MethodRouter,
    get, post,
};
use crate::State;
use crate::AppState;
use async_std::sync::Arc;
use crate::PostMeta;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_post_meta))
        .route("/{id}", get(get_post_metum))
        .route("/", post(add_post_metum))
}

#[axum::debug_handler]
async fn add_post_metum(State(state): State<Arc<AppState>>, Json(post_meta): Json<PostMeta>) -> Result<Json<u32>, RequestError> {
    let service_provider = state.service_provider.lock().await;
    let post_meta_service = service_provider.post_meta_service();

    let post_meta = post_meta_service.add_post_metum(post_meta);

    match post_meta {
        Ok(post_meta) => {
            return Ok(Json(0));
        },
        Err(_) => {
            return Err(RequestError::CreationError);
        }
    }
}

async fn get_post_metum(State(state): State<Arc<AppState>>, Path(id): Path<u32>) -> Result<Json<PostMeta>, RequestError> {

    let service_provider = state.service_provider.lock().await;
    let post_meta_service = service_provider.post_meta_service();

    let post_meta = post_meta_service.get_post_metum(id);

    let Some(post_meta) = post_meta else {
        return Err(RequestError::NotFound("Post Meta".to_string()));
    };

    Ok(Json(post_meta))
}

#[axum::debug_handler]
async fn get_post_meta(State(state): State<Arc<AppState>>, Query(filters): Query<PostmetaFilters>) -> Result<Json<Vec<PostMeta>>, RequestError> {

    let service_provider = state.service_provider.lock().await;
    let post_meta_service = service_provider.post_meta_service();

    let post_meta = post_meta_service.get_post_meta(filters);

    Ok(Json(
        post_meta
    ))

}

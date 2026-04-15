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
use crate::repository::UserMetaFilters;
use axum::extract::Path;
use axum::extract::Query;
use crate::UserMeta;

pub fn router() -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_user_meta))
}



async fn get_user_meta(State(state): State<Arc<AppState>>, Query(filters): Query<UserMetaFilters>) -> Json<Value> {
    
    let service_provider = state.service_provider.lock().await;
    let user_meta_service = service_provider.user_meta_service();

    println!("{filters:?}");

    let user_meta = user_meta_service.get_user_meta(filters);

    Json(json!(
        user_meta
    ))

}

async fn add_user_metum(State(state): State<Arc<AppState>>, Path(user_meta): Path<UserMeta>) -> Json<Value> {

    let service_provider = state.service_provider.lock().await;
    let user_meta_service = service_provider.user_meta_service();

    println!("Someone is lying here");
    user_meta_service.add_user_meta(user_meta);

    Json(json!(
       true 
    ))

}

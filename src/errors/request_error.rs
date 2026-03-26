use crate::json;
use crate::Json;
use axum::response::{
    Response,
    IntoResponse,
};
use http::StatusCode;

pub enum RequestError {
    NotFound(String),
}

impl IntoResponse for RequestError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::NotFound(for_type) => {
                (StatusCode::NOT_FOUND, format!("{for_type} Not Found"))
            }
        };

        (status, Json(json!({ "error": message }))).into_response()

    }
}

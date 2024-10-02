use crate::models::message_response::MessageResponse;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// PING
///
/// This endpoint returns a simple pong message to indicate that the API is responsive.
#[utoipa::path(
    get,
    path = "/",
    responses(
        (status = 200, description = "Pong", body = MessageResponse),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_ping() -> Response {
    let response = MessageResponse {
        message: "Pong".to_string(),
    };
    Json(response).into_response()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_ping))
}
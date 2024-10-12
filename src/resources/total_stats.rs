use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Total stats
///
/// Accumulating interesting information of all data.
#[utoipa::path(
    get,
    path = "/total-stats",
    responses(
        (status = 200, description = "Total Stats", body = TotalStats),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_total_stats(
    State(state): State<Arc<AppState>>,
) -> Response {
    Json(&state.total_stats).into_response()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_total_stats))
}
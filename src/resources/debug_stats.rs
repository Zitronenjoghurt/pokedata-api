use axum::extract::State;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Debug stats
///
/// Accumulating information for finding holes in the dataset, etc.
#[utoipa::path(
    get,
    path = "/debug-stats",
    responses(
        (status = 200, description = "Total Stats", body = DebugStats),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_debug_stats(
    State(state): State<Arc<AppState>>,
) -> Response {
    Json(&state.debug_stats).into_response()
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_debug_stats))
}
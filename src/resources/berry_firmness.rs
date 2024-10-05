use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch berry firmness
///
/// If no ID is provided, all berry firmness will be returned.
#[utoipa::path(
    get,
    path = "/berry-firmness",
    params(IdsQuery),
    responses(
        (status = 200, description = "Berry firmness data", body = BerryFirmnessBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Berries"
)]
async fn get_berry_firmness(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.berry_firmness).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_berry_firmness))
}
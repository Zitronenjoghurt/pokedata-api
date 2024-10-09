use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch location areas
///
/// If no ID is provided, all location areas will be returned.
#[utoipa::path(
    get,
    path = "/location-area",
    params(IdsQuery),
    responses(
        (status = 200, description = "Location area data", body = LocationAreaBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Locations"
)]
async fn get_location_area(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.location_areas).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_location_area))
}
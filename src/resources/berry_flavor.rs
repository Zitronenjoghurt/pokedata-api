use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch berry flavors
///
/// If no ID is provided, all berry flavors will be returned.
#[utoipa::path(
    get,
    path = "/berry-flavor",
    params(IdsQuery),
    responses(
        (status = 200, description = "Berry flavor data", body = BerryFlavorBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Berries"
)]
async fn get_berry_flavor(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.berry_flavors).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_berry_flavor))
}
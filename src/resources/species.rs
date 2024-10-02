use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch species
///
/// If no ID is provided, all species will be returned.
#[utoipa::path(
    get,
    path = "/species",
    params(IdsQuery),
    responses(
        (status = 200, description = "Species data", body = SpeciesBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Species"
)]
async fn get_species(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.species).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_species))
}
use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch encounter methods
///
/// If no ID is provided, all encounter methods will be returned.
#[utoipa::path(
    get,
    path = "/encounter-method",
    params(IdsQuery),
    responses(
        (status = 200, description = "Encounter method data", body = EncounterMethodBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Encounters"
)]
async fn get_encounter_method(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.encounter_methods).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_encounter_method))
}
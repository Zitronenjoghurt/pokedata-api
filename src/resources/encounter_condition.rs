use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch encounter conditions
///
/// If no ID is provided, all encounter conditions will be returned.
#[utoipa::path(
    get,
    path = "/encounter-condition",
    params(IdsQuery),
    responses(
        (status = 200, description = "Encounter condition data", body = EncounterConditionBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Encounters"
)]
async fn get_encounter_condition(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.encounter_conditions).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_encounter_condition))
}
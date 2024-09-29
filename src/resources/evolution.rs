use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::api::evolution::Evolution;
use pokedata_api_entities::app_state::AppState;

/// Fetch evolutions
///
/// If no ID is provided, all evolutions will be returned.
#[utoipa::path(
    get,
    path = "/evolution",
    params(IdsQuery),
    responses(
        (status = 200, description = "Evolution data", body = EvolutionBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Evolutions"
)]
async fn get_evolution(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<Evolution>(query.ids, &state.evolutions).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_evolution))
}
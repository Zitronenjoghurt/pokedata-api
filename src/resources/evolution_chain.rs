use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch evolution chains
///
/// If no ID is provided, all evolution chains will be returned.
#[utoipa::path(
    get,
    path = "/evolution-chain",
    params(IdsQuery),
    responses(
        (status = 200, description = "Evolution chain data", body = EvolutionChainBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Evolutions"
)]
async fn get_evolution_chain(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.evolution_chains).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_evolution_chain))
}
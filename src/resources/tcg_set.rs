use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::api::tcg_set::TcgSet;
use pokedata_api_entities::app_state::AppState;

/// Fetch tcg sets
///
/// If no ID is provided, all tcg sets will be returned.
#[utoipa::path(
    get,
    path = "/tcg-set",
    params(IdsQuery),
    responses(
        (status = 200, description = "Tcg set data", body = TcgSetBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "TCG"
)]
async fn get_tcg_set(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<TcgSet>(query.ids, &state.tcg_sets).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_tcg_set))
}
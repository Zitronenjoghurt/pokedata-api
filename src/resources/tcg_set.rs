use crate::queries::identifier::IdentifierQuery;
use crate::queries::ids::IdsQuery;
use crate::resources::{get_entities, get_using_search_index};
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch tcg sets
///
/// If no ID is provided, all tcg sets will be returned.
#[utoipa::path(
    get,
    path = "/tcg-set",
    params(IdsQuery, IdentifierQuery),
    responses(
        (status = 200, description = "Tcg set data", body = TcgSetBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "TCG"
)]
async fn get_tcg_set(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
    Query(identifier_query): Query<IdentifierQuery>,
) -> Response {
    if let Some(identifier) = identifier_query.identifier {
        get_using_search_index(identifier, &state.tcg_sets, &state.search_indices.set_identifier_to_set_id).await
    } else {
        get_entities(query.ids, &state.tcg_sets).await
    }
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_tcg_set))
}
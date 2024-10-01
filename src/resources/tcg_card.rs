use crate::queries::identifier::IdentifierQuery;
use crate::queries::ids::IdsQuery;
use crate::resources::{get_entities, get_using_search_index};
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch tcg cards
///
/// If no ID is provided, all tcg cards will be returned.
#[utoipa::path(
    get,
    path = "/tcg-card",
    params(IdsQuery, IdentifierQuery),
    responses(
        (status = 200, description = "Tcg card data", body = TcgCardBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 404, description = "Card not found"),
        (status = 500, description = "Server error"),
    ),
    tag = "TCG"
)]
async fn get_tcg_card(
    State(state): State<AppState>,
    Query(ids_query): Query<IdsQuery>,
    Query(identifier_query): Query<IdentifierQuery>,
) -> Response {
    if let Some(identifier) = identifier_query.identifier {
        get_using_search_index(identifier, &state.tcg_cards, &state.search_indices.card_identifier_to_card_id).await
    } else {
        get_entities(ids_query.ids, &state.tcg_cards).await
    }
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_tcg_card))
}
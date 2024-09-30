use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::app_state::AppState;

/// Fetch tcg cards
///
/// If no ID is provided, all tcg cards will be returned.
#[utoipa::path(
    get,
    path = "/tcg-card",
    params(IdsQuery),
    responses(
        (status = 200, description = "Tcg card data", body = TcgCardBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "TCG"
)]
async fn get_tcg_card(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<TcgCard>(query.ids, &state.tcg_cards).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_tcg_card))
}
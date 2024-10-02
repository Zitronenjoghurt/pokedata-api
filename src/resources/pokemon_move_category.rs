use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch pokemon move categories
///
/// If no ID is provided, all pokemon move categories will be returned.
#[utoipa::path(
    get,
    path = "/move-category",
    params(IdsQuery),
    responses(
        (status = 200, description = "Pokemon move category data", body = PokemonMoveCategoryBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Moves"
)]
async fn get_move_category(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.move_categories).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_move_category))
}
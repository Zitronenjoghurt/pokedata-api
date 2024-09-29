use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::api::pokemon_move_category::PokemonMoveCategory;
use pokedata_api_entities::app_state::AppState;

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
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<PokemonMoveCategory>(query.ids, &state.move_categories).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_move_category))
}
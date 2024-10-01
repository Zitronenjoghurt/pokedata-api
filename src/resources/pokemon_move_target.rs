use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch pokemon move targets
///
/// If no ID is provided, all pokemon move targets will be returned.
#[utoipa::path(
    get,
    path = "/move-target",
    params(IdsQuery),
    responses(
        (status = 200, description = "Pokemon move target data", body = PokemonMoveTargetBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Moves"
)]
async fn get_move_target(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.move_targets).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_move_target))
}
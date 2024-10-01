use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch pokemon move flags
///
/// If no ID is provided, all pokemon move flags will be returned.
#[utoipa::path(
    get,
    path = "/move-flag",
    params(IdsQuery),
    responses(
        (status = 200, description = "Pokemon move flag data", body = PokemonMoveFlagBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Moves"
)]
async fn get_move_flag(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.move_flags).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_move_flag))
}
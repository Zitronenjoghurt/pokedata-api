use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch pokemon move damage classes
///
/// If no ID is provided, all pokemon move damage classes will be returned.
#[utoipa::path(
    get,
    path = "/move-damage-class",
    params(IdsQuery),
    responses(
        (status = 200, description = "Pokemon move damage class data", body = PokemonMoveDamageClassBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Moves"
)]
async fn get_move_damage_class(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.move_damage_classes).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_move_damage_class))
}
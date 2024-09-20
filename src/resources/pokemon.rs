use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon::Pokemon;

/// Fetch pokemon
///
/// If no ID is provided, all pokemon will be returned.
#[utoipa::path(
    get,
    path = "/pokemon",
    params(IdsQuery),
    responses(
        (status = 200, description = "Pokemon data", body = PokemonBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_pokemon(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<Pokemon>(query.ids, &state.pokemon).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_pokemon))
}
use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch types
///
/// If no ID is provided, all types will be returned.
#[utoipa::path(
    get,
    path = "/pokemon-type",
    params(IdsQuery),
    responses(
        (status = 200, description = "Type data", body = PokemonTypeBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Types"
)]
async fn get_pokemon_type(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.types).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_pokemon_type))
}
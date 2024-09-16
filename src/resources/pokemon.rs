use crate::models::bulk_response::PokemonBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
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
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_pokemon(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let pokemon = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = PokemonBulkResponse {
        count: pokemon.len(),
        results: pokemon,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<Pokemon> {
    state.pokemon.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<Pokemon> {
    ids.iter()
        .filter_map(|id| state.pokemon.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_pokemon))
}
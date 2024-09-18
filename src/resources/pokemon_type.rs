use crate::models::bulk_response::PokemonTypeBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon_type::PokemonType;

/// Fetch types
///
/// If no ID is provided, all types will be returned.
#[utoipa::path(
    get,
    path = "/pokemon-type",
    params(IdsQuery),
    responses(
        (status = 200, description = "Type data", body = VersionBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_pokemon_type(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let types = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = PokemonTypeBulkResponse {
        count: types.len(),
        results: types,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<PokemonType> {
    state.types.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<PokemonType> {
    ids.iter()
        .filter_map(|id| state.types.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_pokemon_type))
}
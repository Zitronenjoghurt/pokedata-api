use crate::models::bulk_response::HabitatBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon_habitat::PokemonHabitat;

/// Fetch habitats
///
/// If no ID is provided, all habitats will be returned.
#[utoipa::path(
    get,
    path = "/habitat",
    params(IdsQuery),
    responses(
        (status = 200, description = "Habitat data", body = HabitatBulkResponse),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_habitat(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let habitats = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = HabitatBulkResponse {
        count: habitats.len(),
        results: habitats,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<PokemonHabitat> {
    state.habitats.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<PokemonHabitat> {
    ids.iter()
        .filter_map(|id| state.habitats.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_habitat))
}
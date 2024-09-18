use crate::models::bulk_response::ColorBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon_color::PokemonColor;

/// Fetch colors
///
/// If no ID is provided, all colors will be returned.
#[utoipa::path(
    get,
    path = "/color",
    params(IdsQuery),
    responses(
        (status = 200, description = "Color data", body = ColorBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_color(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let colors = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = ColorBulkResponse {
        count: colors.len(),
        results: colors,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<PokemonColor> {
    state.colors.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<PokemonColor> {
    ids.iter()
        .filter_map(|id| state.colors.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_color))
}
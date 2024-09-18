use crate::models::bulk_response::GenerationBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::generation::Generation;

/// Fetch generations
///
/// If no ID is provided, all generations will be returned.
#[utoipa::path(
    get,
    path = "/generation",
    params(IdsQuery),
    responses(
        (status = 200, description = "Generation data", body = ColorBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_generation(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let generations = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = GenerationBulkResponse {
        count: generations.len(),
        results: generations,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<Generation> {
    state.generations.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<Generation> {
    ids.iter()
        .filter_map(|id| state.generations.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_generation))
}
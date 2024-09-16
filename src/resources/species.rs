use crate::models::bulk_response::SpeciesBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::species::Species;

/// Fetch Species.
///
/// If no ID is provided, all species will be returned.
#[utoipa::path(
    get,
    path = "/species",
    params(IdsQuery),
    responses(
        (status = 200, description = "Species data", body = SpeciesBulkResponse),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_species(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let species = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = SpeciesBulkResponse {
        count: species.len(),
        results: species,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<Species> {
    state.species.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<Species> {
    ids.iter()
        .filter_map(|id| state.species.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_species))
}
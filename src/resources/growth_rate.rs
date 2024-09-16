use crate::models::bulk_response::GrowthRateBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::growth_rate::GrowthRate;

/// Fetch growth rates
///
/// If no ID is provided, all growth rates will be returned.
#[utoipa::path(
    get,
    path = "/growth-rate",
    params(IdsQuery),
    responses(
        (status = 200, description = "Growth rate data", body = GrowthRateBulkResponse),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_growth_rate(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let growth_rates = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = GrowthRateBulkResponse {
        count: growth_rates.len(),
        results: growth_rates,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<GrowthRate> {
    state.growth_rates.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<GrowthRate> {
    ids.iter()
        .filter_map(|id| state.growth_rates.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_growth_rate))
}
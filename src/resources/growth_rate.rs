use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch growth rates
///
/// If no ID is provided, all growth rates will be returned.
#[utoipa::path(
    get,
    path = "/growth-rate",
    params(IdsQuery),
    responses(
        (status = 200, description = "Growth rate data", body = GrowthRateBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Growth Rates"
)]
async fn get_growth_rate(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.growth_rates).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_growth_rate))
}
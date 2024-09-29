use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::api::growth_rate::GrowthRate;
use pokedata_api_entities::app_state::AppState;

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
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<GrowthRate>(query.ids, &state.growth_rates).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_growth_rate))
}
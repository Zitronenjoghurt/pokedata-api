use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch pokemon stats
///
/// If no ID is provided, all stats will be returned.
#[utoipa::path(
    get,
    path = "/stat",
    params(IdsQuery),
    responses(
        (status = 200, description = "Stat data", body = StatBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Pokemon Stats"
)]
async fn get_stat(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.stats).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_stat))
}
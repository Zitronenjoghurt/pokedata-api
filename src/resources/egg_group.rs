use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch egg groups
///
/// If no ID is provided, all egg groups will be returned.
#[utoipa::path(
    get,
    path = "/egg-group",
    params(IdsQuery),
    responses(
        (status = 200, description = "Egg group data", body = EggGroupBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Egg Groups"
)]
async fn get_egg_group(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.egg_groups).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_egg_group))
}
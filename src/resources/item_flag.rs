use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch item flags
///
/// If no ID is provided, all item flags will be returned.
#[utoipa::path(
    get,
    path = "/item-flag",
    params(IdsQuery),
    responses(
        (status = 200, description = "Item flag data", body = ItemFlagBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Items"
)]
async fn get_item_flag(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.item_flags).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_item_flag))
}
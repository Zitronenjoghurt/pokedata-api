use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch item categories
///
/// If no ID is provided, all item categories will be returned.
#[utoipa::path(
    get,
    path = "/item-category",
    params(IdsQuery),
    responses(
        (status = 200, description = "Item category data", body = ItemCategoryBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Items"
)]
async fn get_item_category(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.item_categories).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_item_category))
}
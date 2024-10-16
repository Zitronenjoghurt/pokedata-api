use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch pokemon shapes
///
/// If no ID is provided, all pokemon shapes will be returned.
#[utoipa::path(
    get,
    path = "/shape",
    params(IdsQuery),
    responses(
        (status = 200, description = "Shape data", body = ShapeBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Pokemon"
)]
async fn get_shape(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.shapes).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_shape))
}
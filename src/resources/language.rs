use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;
use std::sync::Arc;

/// Fetch languages
///
/// If no ID is provided, all languages will be returned.
#[utoipa::path(
    get,
    path = "/language",
    params(IdsQuery),
    responses(
        (status = 200, description = "Species data", body = LanguageBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Languages"
)]
async fn get_language(
    State(state): State<Arc<AppState>>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.languages).await
}

pub fn router() -> Router<Arc<AppState>> {
    Router::new().route("/", get(get_language))
}
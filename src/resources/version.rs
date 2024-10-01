use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch versions
///
/// If no ID is provided, all versions will be returned.
#[utoipa::path(
    get,
    path = "/version",
    params(IdsQuery),
    responses(
        (status = 200, description = "Version data", body = VersionBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Versions"
)]
async fn get_version(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.versions).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_version))
}
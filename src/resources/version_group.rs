use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::api::version_group::VersionGroup;
use pokedata_api_entities::app_state::AppState;

/// Fetch version groups
///
/// If no ID is provided, all version groups will be returned.
#[utoipa::path(
    get,
    path = "/version-group",
    params(IdsQuery),
    responses(
        (status = 200, description = "Version group data", body = VersionGroupBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Version Groups"
)]
async fn get_version_group(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<VersionGroup>(query.ids, &state.version_groups).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_version_group))
}
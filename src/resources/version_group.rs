use crate::models::bulk_response::VersionGroupBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::version_group::VersionGroup;

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
    tag = "Misc"
)]
async fn get_version_group(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let groups = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = VersionGroupBulkResponse {
        count: groups.len(),
        results: groups,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<VersionGroup> {
    state.version_groups.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<VersionGroup> {
    ids.iter()
        .filter_map(|id| state.version_groups.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_version_group))
}
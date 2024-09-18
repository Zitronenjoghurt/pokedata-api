use crate::models::bulk_response::VersionBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::version::Version;

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
    tag = "Misc"
)]
async fn get_version(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let versions = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = VersionBulkResponse {
        count: versions.len(),
        results: versions,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<Version> {
    state.versions.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<Version> {
    ids.iter()
        .filter_map(|id| state.versions.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_version))
}
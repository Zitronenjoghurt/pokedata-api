use crate::models::bulk_response::LanguageBulkResponse;
use crate::queries::ids::IdsQuery;
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::language::Language;

/// Fetch languages
///
/// If no ID is provided, all languages will be returned.
#[utoipa::path(
    get,
    path = "/language",
    params(IdsQuery),
    responses(
        (status = 200, description = "Species data", body = LanguageBulkResponse),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_language(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    let ids = query.ids.unwrap_or_default();

    let languages = if ids.is_empty() {
        get_all(&state).await
    } else {
        get_specific(&state, ids).await
    };

    let response = LanguageBulkResponse {
        count: languages.len(),
        results: languages,
    };

    Json(response).into_response()
}

async fn get_all(state: &AppState) -> Vec<Language> {
    state.languages.values().cloned().collect()
}

async fn get_specific(state: &AppState, ids: Vec<u32>) -> Vec<Language> {
    ids.iter()
        .filter_map(|id| state.languages.get(id).cloned())
        .collect()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_language))
}
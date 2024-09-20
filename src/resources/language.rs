use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
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
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_language(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<Language>(query.ids, &state.languages).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_language))
}
use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_entities::app_state::AppState;

/// Fetch habitats
///
/// If no ID is provided, all habitats will be returned.
#[utoipa::path(
    get,
    path = "/habitat",
    params(IdsQuery),
    responses(
        (status = 200, description = "Habitat data", body = HabitatBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Habitats"
)]
async fn get_habitat(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities(query.ids, &state.habitats).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_habitat))
}
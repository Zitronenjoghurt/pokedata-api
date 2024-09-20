use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::species::Species;

/// Fetch species
///
/// If no ID is provided, all species will be returned.
#[utoipa::path(
    get,
    path = "/species",
    params(IdsQuery),
    responses(
        (status = 200, description = "Species data", body = SpeciesBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_species(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<Species>(query.ids, &state.species).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_species))
}
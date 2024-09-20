use crate::queries::ids::IdsQuery;
use crate::resources::get_entities;
use axum::extract::{Query, State};
use axum::response::Response;
use axum::routing::get;
use axum::Router;
use pokedata_api_types::app_state::AppState;
use pokedata_api_types::entities::api::pokemon_shape::PokemonShape;

/// Fetch shapes
///
/// If no ID is provided, all shapes will be returned.
#[utoipa::path(
    get,
    path = "/shape",
    params(IdsQuery),
    responses(
        (status = 200, description = "Shape data", body = ShapeBulkResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_shape(
    State(state): State<AppState>,
    Query(query): Query<IdsQuery>,
) -> Response {
    get_entities::<PokemonShape>(query.ids, &state.shapes).await
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new().route("/", get(get_shape))
}
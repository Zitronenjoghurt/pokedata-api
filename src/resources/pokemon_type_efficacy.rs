use crate::models::type_effectiveness::{AllTypeEffectivenessResponse, TypeEffectivenessResponse};
use crate::queries::type_effectiveness::{AllTypeEffectivenessQuery, TypeEffectivenessQuery};
use axum::extract::{Query, State};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use pokedata_api_types::app_state::AppState;

/// Get the damage factor of a type against a given type combination
///
/// All factors are multiplied by *100 to fit into integers
#[utoipa::path(
    get,
    path = "/type-efficacy",
    params(TypeEffectivenessQuery),
    responses(
        (status = 200, description = "Type effectiveness", body = TypeEffectivenessResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_type_efficacy(
    State(state): State<AppState>,
    Query(query): Query<TypeEffectivenessQuery>,
) -> Response {
    let attacking_id = query.attacking_id;
    let defending_ids = query.defending_ids;
    let generation_id = query.generation_id.unwrap_or(state.latest_generation);

    let effectiveness = state.type_efficacies.get_damage_factor_multi_defenders(generation_id, attacking_id, defending_ids);
    let response = TypeEffectivenessResponse(effectiveness);

    Json(response).into_response()
}

/// Get all damage factors against a given type combinations
///
/// All factors are multiplied by *100 to fit into integers
#[utoipa::path(
    get,
    path = "/type-efficacy/all",
    params(AllTypeEffectivenessQuery),
    responses(
        (status = 200, description = "Type effectiveness", body = AllTypeEffectivenessResponse),
        (status = 400, description = "Invalid parameters"),
        (status = 500, description = "Server error"),
    ),
    tag = "Misc"
)]
async fn get_type_efficacy_all(
    State(state): State<AppState>,
    Query(query): Query<AllTypeEffectivenessQuery>,
) -> Response {
    let major_type_ids = state.major_type_ids;
    let defending_ids = query.defending_ids;
    let generation_id = query.generation_id.unwrap_or(state.latest_generation);

    let efficacies = state.type_efficacies.get_damage_factors_multi_attackers(generation_id, major_type_ids, defending_ids);
    let response = AllTypeEffectivenessResponse(efficacies);

    Json(response).into_response()
}

pub fn router() -> Router<AppState> {
    Router::<AppState>::new()
        .route("/", get(get_type_efficacy))
        .route("/all", get(get_type_efficacy_all))
}
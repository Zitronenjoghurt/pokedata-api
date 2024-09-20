use crate::models::bulk_response::BulkResponse;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use std::collections::HashMap;
use utoipa::ToSchema;

pub mod color;
pub mod generation;
pub mod growth_rate;
pub mod habitat;
pub mod language;
pub mod ping;
pub mod pokemon;
pub mod pokemon_type;
pub mod pokemon_type_efficacy;
pub mod shape;
pub mod species;
pub mod version;
pub mod version_group;

/// A generalized handler to get entities by ids
pub async fn get_entities<T: Clone + Serialize + for<'s> ToSchema<'s>>(
    ids: Option<Vec<u32>>,
    entities: &HashMap<u32, T>,
) -> Response {
    let ids = ids.unwrap_or_default();

    let results: Vec<T> = if ids.is_empty() {
        entities.values().cloned().collect()
    } else {
        ids.iter()
            .filter_map(|id| entities.get(id).cloned())
            .collect()
    };

    let response = BulkResponse {
        count: results.len(),
        results,
    };

    Json(response).into_response()
}
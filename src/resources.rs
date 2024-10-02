use crate::models::bulk_response::BulkResponse;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use pokedata_api_entities::data_structures::search_index::SearchIndex;
use pokedata_api_entities::traits::has_id::HasId;
use serde::Serialize;
use std::collections::HashMap;
use std::hash::Hash;
use utoipa::ToSchema;

pub mod color;
pub mod evolution;
pub mod evolution_chain;
pub mod generation;
pub mod growth_rate;
pub mod habitat;
pub mod language;
pub mod ping;
pub mod pokemon;
pub mod pokemon_move;
pub mod pokemon_move_ailment;
pub mod pokemon_move_category;
pub mod pokemon_move_damage_class;
pub mod pokemon_move_flag;
pub mod pokemon_move_target;
pub mod pokemon_type;
pub mod pokemon_type_efficacy;
pub mod region;
pub mod shape;
pub mod species;
pub mod stat;
pub mod tcg_card;
pub mod tcg_set;
pub mod version;
pub mod version_group;

/// A generalized handler to get entities by ids
pub async fn get_entities<T: Clone + HasId + Serialize + for<'s> ToSchema<'s>>(
    ids: Option<Vec<i32>>,
    entities: &HashMap<i32, T>,
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

pub async fn get_using_search_index<E, K, I>(
    identifier: I,
    entities: &HashMap<K, E>,
    search_index: &SearchIndex<I, K>,
) -> Response
where
    E: Clone + HasId + Serialize + for<'s> ToSchema<'s>,
    I: Eq + Hash,
    K: Eq + Hash,
{
    if let Some(id) = search_index.get(&identifier) {
        if let Some(result) = entities.get(id) {
            return Json(result).into_response();
        }
    }

    (
        StatusCode::NOT_FOUND,
        "Requested entity not found.",
    ).into_response()
}
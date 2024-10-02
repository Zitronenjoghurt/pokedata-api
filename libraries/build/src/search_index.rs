use crate::search_index::card_identifier_to_card_id::index_card_identifier_to_card_id;
use crate::search_index::set_identifier_to_set_id::index_set_identifier_to_set_id;
use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::api::tcg_set::TcgSet;
use pokedata_api_entities::search_indices::SearchIndices;
use std::collections::HashMap;

mod card_identifier_to_card_id;
mod set_identifier_to_set_id;

pub fn build_search_indices(
    cards: &HashMap<i32, TcgCard>,
    sets: &HashMap<i32, TcgSet>,
) -> SearchIndices {
    SearchIndices {
        card_identifier_to_card_id: index_card_identifier_to_card_id(cards),
        set_identifier_to_set_id: index_set_identifier_to_set_id(sets),
    }
}
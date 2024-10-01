use crate::api::tcg_card::TcgCard;
use crate::api::tcg_set::TcgSet;
use crate::data_structures::search_index::SearchIndex;
use crate::search_indices::card_identifier_to_card_id::index_card_identifier_to_card_id;
use crate::search_indices::set_identifier_to_set_id::index_set_identifier_to_set_id;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod card_identifier_to_card_id;
mod set_identifier_to_set_id;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SearchIndices {
    pub card_identifier_to_card_id: SearchIndex<String, i32>,
    pub set_identifier_to_set_id: SearchIndex<String, i32>,
}

impl SearchIndices {
    pub fn build(
        cards: &HashMap<i32, TcgCard>,
        sets: &HashMap<i32, TcgSet>,
    ) -> Self {
        Self {
            card_identifier_to_card_id: index_card_identifier_to_card_id(cards),
            set_identifier_to_set_id: index_set_identifier_to_set_id(sets),
        }
    }
}
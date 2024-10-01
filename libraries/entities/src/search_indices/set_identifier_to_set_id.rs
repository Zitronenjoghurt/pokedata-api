use crate::api::tcg_set::TcgSet;
use crate::data_structures::search_index::SearchIndex;
use std::collections::HashMap;

pub fn index_set_identifier_to_set_id(sets: &HashMap<i32, TcgSet>) -> SearchIndex<String, i32> {
    let mut search_index: SearchIndex<String, i32> = SearchIndex::new();
    for (set_id, set) in sets {
        search_index.insert_case_insensitive(set.clone().identifier, *set_id);
    }
    search_index
}
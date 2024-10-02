use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::data_structures::search_index::SearchIndex;
use std::collections::HashMap;

pub fn index_card_identifier_to_card_id(cards: &HashMap<i32, TcgCard>) -> SearchIndex<String, i32> {
    let mut search_index: SearchIndex<String, i32> = SearchIndex::new();
    for (card_id, card) in cards {
        search_index.insert_case_insensitive(card.clone().identifier, *card_id);
    }
    search_index
}
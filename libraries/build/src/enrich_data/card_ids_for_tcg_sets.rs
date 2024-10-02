use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::api::tcg_set::TcgSet;
use pokedata_api_entities::data_structures::search_index::SearchIndex;
use std::collections::HashMap;

pub fn enrich_tcg_sets_with_card_ids(
    cards: &HashMap<i32, TcgCard>,
    sets: &mut HashMap<i32, TcgSet>,
    set_identifier_to_set_id: &SearchIndex<String, i32>,
)
{
    for card in cards.values() {
        let Some(set_identifier) = &card.set_identifier else { continue };
        let Some(set_id) = set_identifier_to_set_id.get(set_identifier) else { continue };
        let Some(set) = sets.get_mut(set_id) else { continue };
        set.card_ids.push(card.id);
    }
}
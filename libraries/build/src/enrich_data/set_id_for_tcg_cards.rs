use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::data_structures::search_index::SearchIndex;
use std::collections::HashMap;

pub fn enrich_tcg_cards_with_set_id(
    cards: &mut HashMap<i32, TcgCard>,
    set_identifier_to_set_id: &SearchIndex<String, i32>,
)
{
    for card in cards.values_mut() {
        let Some(set_identifier) = &card.set_identifier else { continue };
        let Some(set_id) = set_identifier_to_set_id.get(set_identifier) else { continue };
        card.set_id = Some(*set_id);
    }
}
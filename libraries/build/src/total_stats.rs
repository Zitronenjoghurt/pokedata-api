use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::api::tcg_set::TcgSet;
use pokedata_api_entities::total_stats::TotalStats;
use std::collections::HashMap;

pub fn build_total_stats(
    cards: &HashMap<i32, TcgCard>,
    sets: &HashMap<i32, TcgSet>,
) -> TotalStats {
    TotalStats {
        tcg_cards_count: cards.len(),
        tcg_sets_count: sets.len(),
    }
}
use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::total_stats::TotalStats;
use std::collections::HashMap;

pub fn build_total_stats(
    cards: &HashMap<i32, TcgCard>,
) -> TotalStats {
    TotalStats {
        pokemon_tcg_card_count: cards.iter().filter(|(_, card)| card.super_type == *"Pokémon").count(),
        trainer_tcg_card_count: cards.iter().filter(|(_, card)| card.super_type == *"Trainer").count(),
        energy_tcg_card_count: cards.iter().filter(|(_, card)| card.super_type == *"Energy").count(),
    }
}
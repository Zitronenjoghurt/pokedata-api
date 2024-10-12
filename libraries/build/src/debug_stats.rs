use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::debug_stats::DebugStats;
use std::collections::HashMap;

pub fn build_debug_stats(
    cards: &HashMap<i32, TcgCard>
) -> DebugStats {
    DebugStats {
        tcg_cards_missing_dex_numbers: find_pokemon_cards_with_missing_dex_numbers(cards),
    }
}

fn find_pokemon_cards_with_missing_dex_numbers(cards: &HashMap<i32, TcgCard>) -> Vec<String> {
    let mut cards_wo_national_pokedex_numbers: Vec<String> = cards.iter()
        .filter(|(_, card)| card.species_ids.is_empty() && card.super_type == *"Pokémon")
        .map(|(_, card)| card.identifier.clone())
        .collect::<Vec<String>>();
    cards_wo_national_pokedex_numbers.sort_unstable();
    cards_wo_national_pokedex_numbers
}
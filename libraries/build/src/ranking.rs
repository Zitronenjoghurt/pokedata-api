use pokedata_api_entities::api::species::Species;
use pokedata_api_entities::data_structures::ranking::Ranking;
use pokedata_api_entities::rankings::Rankings;
use std::collections::HashMap;

pub fn build_rankings(
    species: &HashMap<i32, Species>
) -> Rankings {
    Rankings {
        species_tcg_card_count: Ranking::create(species, |s| s.tcg_card_ids.len(), true),
        species_tcg_set_count: Ranking::create(species, |s| s.tcg_set_ids.len(), true),
    }
}
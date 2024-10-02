use pokedata_api_entities::api::species::Species;
use pokedata_api_entities::api::tcg_card::TcgCard;
use std::collections::HashMap;

pub fn enrich_species_with_tcg_ids(
    cards: &HashMap<i32, TcgCard>,
    species: &mut HashMap<i32, Species>,
)
{
    for card in cards.values() {
        let species_ids = &card.species_ids;
        for species_id in species_ids {
            let Some(species) = species.get_mut(species_id) else { continue };
            species.tcg_card_ids.push(card.id);
            if let Some(set_id) = card.set_id {
                species.tcg_set_ids.insert(set_id);
            }
        }
    }
}
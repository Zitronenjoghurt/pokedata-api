use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesCSV {
    pub id: Option<u32>,
    pub identifier: Option<String>,
    pub generation_id: Option<u32>,
    pub evolves_from_species_id: Option<u32>,
    pub evolution_chain_id: Option<u32>,
    pub color_id: Option<u32>,
    pub shape_id: Option<u32>,
    pub habitat_id: Option<u32>,
    pub gender_rate: Option<i32>,
    pub capture_rate: Option<u32>,
    pub base_happiness: Option<u32>,
    pub is_baby: Option<u32>,
    pub hatch_counter: Option<u32>,
    pub has_gender_differences: Option<u32>,
    pub growth_rate_id: Option<u32>,
    pub forms_switchable: Option<u32>,
    pub is_legendary: Option<u32>,
    pub is_mythical: Option<u32>,
    pub order: Option<u32>,
    pub conquest_order: Option<u32>,
}

impl CSVEntity for PokemonSpeciesCSV {
    fn file_name() -> &'static str {
        "pokemon_species"
    }
}
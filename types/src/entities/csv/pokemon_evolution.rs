use crate::entities::api::evolution::Evolution;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonEvolutionCSV {
    pub id: u32,
    pub evolved_species_id: u32,
    pub evolution_trigger_id: u32,
    pub trigger_item_id: Option<u32>,
    pub minimum_level: Option<u32>,
    pub gender_id: Option<u32>,
    pub location_id: Option<u32>,
    pub held_item_id: Option<u32>,
    pub time_of_day: Option<String>,
    pub known_move_id: Option<u32>,
    pub known_move_type_id: Option<u32>,
    pub minimum_happiness: Option<u32>,
    pub minimum_beauty: Option<u32>,
    pub minimum_affection: Option<u32>,
    pub relative_physical_stats: Option<i32>,
    pub party_species_id: Option<u32>,
    pub party_type_id: Option<u32>,
    pub trade_species_id: Option<u32>,
    pub needs_overworld_rain: Option<u32>,
    pub turn_upside_down: Option<u32>,
}

impl CSVEntity for PokemonEvolutionCSV {
    fn file_name() -> &'static str {
        "pokemon_evolution"
    }
}

impl ApiCSVEntity for PokemonEvolutionCSV {
    type ApiType = Evolution;
    type ConversionData = ();

    fn convert(entry: Self, _data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Evolution {
            id: entry.id,
            evolved_species_id: entry.evolved_species_id,
            evolution_trigger_id: entry.evolution_trigger_id,
            trigger_item_id: entry.trigger_item_id,
            minimum_level: entry.minimum_level,
            gender_id: entry.gender_id,
            location_id: entry.location_id,
            held_item_id: entry.held_item_id,
            time_of_day: entry.time_of_day,
            known_move_id: entry.known_move_id,
            known_move_type_id: entry.known_move_type_id,
            minimum_happiness: entry.minimum_happiness,
            minimum_beauty: entry.minimum_beauty,
            minimum_affection: entry.minimum_affection,
            relative_physical_stats: entry.relative_physical_stats,
            party_species_id: entry.party_species_id,
            party_type_id: entry.party_type_id,
            trade_species_id: entry.trade_species_id,
            needs_overworld_rain: entry.needs_overworld_rain.unwrap_or_default() == 1,
            turn_upside_down: entry.turn_upside_down.unwrap_or_default() == 1,
        })
    }
}
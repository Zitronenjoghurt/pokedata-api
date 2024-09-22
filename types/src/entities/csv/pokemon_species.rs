use crate::entities::api::localized_values::{LocalizedValuesMap, VersionedLocalizedValuesMap};
use crate::entities::api::pokemon::Pokemon;
use crate::entities::api::species::Species;
use crate::entities::csv::pokemon_species_flavor_text::PokemonSpeciesFlavorTextCSV;
use crate::entities::csv::pokemon_species_names::{PokemonSpeciesGeneraCSV, PokemonSpeciesNamesCSV};
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use crate::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use crate::entities::traits::into_versioned_localized_values_map::IntoVersionedLocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesCSV {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
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

impl ApiCSVEntity for PokemonSpeciesCSV {
    type ApiType = Species;
    type ConversionData = PokemonSpeciesConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Species {
            id: entry.id,
            identifier: entry.identifier,
            gender_rate: entry.gender_rate.unwrap_or(-1),
            capture_rate: entry.capture_rate.unwrap_or(0),
            base_happiness: entry.base_happiness.unwrap_or(0),
            is_baby: entry.is_baby.unwrap_or(0) == 1,
            hatch_counter: entry.hatch_counter.unwrap_or(0),
            has_gender_differences: entry.has_gender_differences.unwrap_or(0) == 1,
            forms_switchable: entry.forms_switchable.unwrap_or(0) == 1,
            is_legendary: entry.is_legendary.unwrap_or(0) == 1,
            is_mythical: entry.is_mythical.unwrap_or(0) == 1,
            dex_order: entry.order.unwrap_or(0),
            conquest_order: entry.conquest_order,
            names: data.names_map.get(entry.id),
            genus: data.genera_map.get(entry.id),
            flavor_texts: data.flavor_text_map.get(entry.id),
            color_id: entry.color_id,
            habitat_id: entry.habitat_id,
            growth_rate_id: entry.growth_rate_id,
            shape_id: entry.shape_id,
            pokemon_ids: get_pokemon_ids_by_species(&data.pokemon_map, entry.id),
            evolution_chain_id: entry.evolution_chain_id,
            evolves_from_species_id: entry.evolves_from_species_id,
        })
    }
}

fn get_pokemon_ids_by_species(pokemon_map: &HashMap<u32, Pokemon>, species_id: u32) -> Vec<u32> {
    pokemon_map
        .values()
        .filter(|pokemon| pokemon.species_id == species_id)
        .map(|pokemon| pokemon.id)
        .collect()
}

#[derive(Default)]
pub struct PokemonSpeciesConversionData {
    pub pokemon_map: HashMap<u32, Pokemon>,
    pub names_map: LocalizedValuesMap,
    pub genera_map: LocalizedValuesMap,
    pub flavor_text_map: VersionedLocalizedValuesMap,
}

impl PokemonSpeciesConversionData {
    pub fn load(data_path: &PathBuf, pokemon: &HashMap<u32, Pokemon>) -> Self {
        Self {
            pokemon_map: pokemon.clone(),
            names_map: PokemonSpeciesNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            genera_map: PokemonSpeciesGeneraCSV::load(data_path).unwrap().into_localized_values_map(),
            flavor_text_map: PokemonSpeciesFlavorTextCSV::load(data_path).unwrap().into_versioned_localized_values_map(),
        }
    }
}
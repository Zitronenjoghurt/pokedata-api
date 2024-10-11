use crate::csv::pokemon_dex_numbers::PokemonDexNumbersCSV;
use crate::csv::pokemon_species_flavor_text::PokemonSpeciesFlavorTextCSV;
use crate::csv::pokemon_species_names::{PokemonSpeciesGeneraCSV, PokemonSpeciesNamesCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing_mapped::HashMapGroupById;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use crate::traits::into_versioned_localized_values_map::IntoVersionedLocalizedValuesMap;
use pokedata_api_entities::api::localized_values::{LocalizedValuesMap, VersionedLocalizedValuesMap};
use pokedata_api_entities::api::pokemon::Pokemon;
use pokedata_api_entities::api::species::Species;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonSpeciesCSV {
    pub id: i32,
    pub identifier: String,
    pub generation_id: i32,
    pub evolves_from_species_id: Option<i32>,
    pub evolution_chain_id: Option<i32>,
    pub color_id: Option<i32>,
    pub shape_id: Option<i32>,
    pub habitat_id: Option<i32>,
    pub gender_rate: Option<i32>,
    pub capture_rate: Option<i32>,
    pub base_happiness: Option<i32>,
    pub is_baby: Option<i32>,
    pub hatch_counter: Option<i32>,
    pub has_gender_differences: Option<i32>,
    pub growth_rate_id: Option<i32>,
    pub forms_switchable: Option<i32>,
    pub is_legendary: Option<i32>,
    pub is_mythical: Option<i32>,
    pub order: Option<i32>,
    pub conquest_order: Option<i32>,
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
            tcg_card_ids: vec![],
            tcg_set_ids: HashSet::new(),
            dex_numbers: data.dex_number_map.get(&entry.id).cloned().unwrap_or_default(),
        })
    }
}

fn get_pokemon_ids_by_species(pokemon_map: &HashMap<i32, Pokemon>, species_id: i32) -> Vec<i32> {
    pokemon_map
        .values()
        .filter(|pokemon| pokemon.species_id == species_id)
        .map(|pokemon| pokemon.id)
        .collect()
}

#[derive(Default)]
pub struct PokemonSpeciesConversionData {
    pub pokemon_map: HashMap<i32, Pokemon>,
    pub names_map: LocalizedValuesMap,
    pub genera_map: LocalizedValuesMap,
    pub flavor_text_map: VersionedLocalizedValuesMap,
    pub dex_number_map: HashMap<i32, HashMap<i32, i32>>,
}

impl PokemonSpeciesConversionData {
    pub fn load(data_path: &PathBuf, pokemon: &HashMap<i32, Pokemon>) -> Self {
        Self {
            pokemon_map: pokemon.clone(),
            names_map: PokemonSpeciesNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            genera_map: PokemonSpeciesGeneraCSV::load(data_path).unwrap().into_localized_values_map(),
            flavor_text_map: PokemonSpeciesFlavorTextCSV::load(data_path).unwrap().into_versioned_localized_values_map(),
            dex_number_map: PokemonDexNumbersCSV::load(data_path).unwrap().group_by_id_mapped(),
        }
    }
}
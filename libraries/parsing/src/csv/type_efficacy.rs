use crate::csv::type_efficacy_past::TypeEfficacyPastCSV;
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::pokemon_type_efficacy::{PokemonTypeEfficacies, PokemonTypeEfficacyEntry};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TypeEfficacyCSV {
    pub damage_type_id: i32,
    pub target_type_id: i32,
    pub damage_factor: i32,
}

impl CSVEntity for TypeEfficacyCSV {
    fn file_name() -> &'static str {
        "type_efficacy"
    }
}

impl ApiCSVEntity for TypeEfficacyCSV {
    type ApiType = PokemonTypeEfficacyEntry;
    type ConversionData = TypeEfficacyConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        let mut efficacy_entry = PokemonTypeEfficacyEntry {
            damage_type_id: entry.damage_type_id,
            target_type_id: entry.target_type_id,
            damage_factor: entry.damage_factor,
        };

        // Apply past change if it affects the current generation
        for past_entry in &data.type_efficacy_past {
            if past_entry.damage_type_id == entry.damage_type_id &&
                past_entry.target_type_id == entry.target_type_id &&
                past_entry.generation_id >= data.current_gen
            {
                efficacy_entry.damage_factor = past_entry.damage_factor;
                break;
            }
        }

        Ok(efficacy_entry)
    }
}

pub fn build_efficacies_by_generation(
    data_path: &PathBuf,
    latest_gen: i32,
) -> Result<PokemonTypeEfficacies, Box<dyn Error>> {
    let past_efficacies = TypeEfficacyPastCSV::load(data_path).unwrap();
    let mut efficacies_by_gen: HashMap<i32, Vec<PokemonTypeEfficacyEntry>> = HashMap::new();

    for gen in (1..=latest_gen).rev() {
        let conversion_data = TypeEfficacyConversionData::new(past_efficacies.clone(), gen);
        let efficacies = TypeEfficacyCSV::load_and_convert(data_path, &conversion_data)?;
        efficacies_by_gen.insert(gen, efficacies);
    }

    Ok(PokemonTypeEfficacies(efficacies_by_gen))
}

#[derive(Default)]
pub struct TypeEfficacyConversionData {
    pub type_efficacy_past: Vec<TypeEfficacyPastCSV>,
    pub current_gen: i32,
}

impl TypeEfficacyConversionData {
    pub fn new(past_efficacies: Vec<TypeEfficacyPastCSV>, current_gen: i32) -> Self {
        Self {
            type_efficacy_past: past_efficacies,
            current_gen,
        }
    }
}
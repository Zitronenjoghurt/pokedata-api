use crate::entities::api::pokemon::Pokemon;
use crate::entities::api::pokemon_stats::PokemonStats;
use crate::entities::api::type_slots::{TypeSlots, TypeSlotsPast};
use crate::entities::csv::pokemon_game_indices::PokemonVersionIdsCSV;
use crate::entities::csv::pokemon_stats::PokemonStatsCSV;
use crate::entities::csv::pokemon_types::PokemonTypesCSV;
use crate::entities::csv::pokemon_types_past::PokemonTypesPastCSV;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::sprites::SpriteIndex;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use crate::entities::traits::id_value_pairing::GroupById;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonCSV {
    pub id: u32,
    pub identifier: String,
    pub species_id: u32,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub base_experience: Option<u32>,
    pub order: Option<u32>,
    pub is_default: Option<u32>,
}

impl CSVEntity for PokemonCSV {
    fn file_name() -> &'static str {
        "pokemon"
    }
}

impl ApiCSVEntity for PokemonCSV {
    type ApiType = Pokemon;
    type ConversionData = PokemonConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        let (sprites, form_sprites) = data.sprite_index.get_by_id(entry.id).unwrap_or_default();

        Ok(
            Pokemon {
                id: entry.id,
                identifier: entry.identifier,
                species_id: entry.species_id,
                height_decimeter: entry.height.unwrap_or_default(),
                weight_hectograms: entry.weight.unwrap_or_default(),
                base_experience: entry.base_experience.unwrap_or_default(),
                dex_order: entry.order.unwrap_or_default(),
                is_default: entry.is_default.unwrap_or_default() == 1,
                stats: data.stats_map.get(&entry.id).cloned().unwrap_or_default(),
                types: data.type_slots_map.get(&entry.id).cloned().unwrap_or_default(),
                types_past: data.type_slots_past_map.get(&entry.id).cloned(),
                version_ids: data.version_id_map.get(&entry.id).cloned().unwrap_or_default(),
                sprites,
                form_sprites,
            }
        )
    }
}

#[derive(Debug)]
pub struct PokemonConversionData {
    pub version_id_map: HashMap<u32, Vec<u32>>,
    pub stats_map: HashMap<u32, PokemonStats>,
    pub type_slots_map: HashMap<u32, TypeSlots>,
    pub type_slots_past_map: HashMap<u32, TypeSlotsPast>,
    pub sprite_index: Arc<SpriteIndex>,
}

impl PokemonConversionData {
    pub fn load(data_path: &PathBuf, sprite_index: Arc<SpriteIndex>) -> Self {
        let stats = PokemonStatsCSV::load(data_path).unwrap();
        let types = PokemonTypesCSV::load(data_path).unwrap();
        let types_past = PokemonTypesPastCSV::load(data_path).unwrap();
        Self {
            version_id_map: PokemonVersionIdsCSV::load(data_path).unwrap().group_by_id(),
            stats_map: PokemonStatsCSV::map(stats),
            type_slots_map: PokemonTypesCSV::map(types),
            type_slots_past_map: PokemonTypesPastCSV::map(types_past),
            sprite_index,
        }
    }
}
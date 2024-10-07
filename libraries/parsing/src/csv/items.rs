use crate::csv::item_flag_map::ItemFlagMapCSV;
use crate::csv::item_flavor_text::ItemFlavorTextCSV;
use crate::csv::item_game_indices::{ItemGameIndicesCSV, ItemGenerationIdsCSV};
use crate::csv::item_names::ItemNamesCSV;
use crate::csv::items_prose::{ItemEffectsCSV, ItemShortEffectsCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::id_value_pairing::GroupById;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use crate::traits::into_version_grouped_localized_values_map::IntoVersionGroupedLocalizedValuesMap;
use pokedata_api_entities::api::item::Item;
use pokedata_api_entities::api::localized_values::{LocalizedValuesMap, VersionGroupedLocalizedValuesMap};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemsCSV {
    pub id: i32,
    pub identifier: String,
    pub category_id: i32,
    pub cost: i32,
    pub fling_power: Option<i32>,
    pub fling_effect_id: Option<i32>,
}

impl CSVEntity for ItemsCSV {
    fn file_name() -> &'static str {
        "items"
    }
}

impl ApiCSVEntity for ItemsCSV {
    type ApiType = Item;
    type ConversionData = ItemConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        let game_indices: HashSet<i32> = data.game_indices_map.get(&entry.id).cloned().unwrap_or_default().into_iter().collect();
        let generation_ids: HashSet<i32> = data.generation_ids_map.get(&entry.id).cloned().unwrap_or_default().into_iter().collect();

        Ok(Item {
            id: entry.id,
            identifier: entry.identifier,
            category_id: entry.category_id,
            cost: entry.cost,
            flag_ids: data.flag_ids_map.get(&entry.id).cloned().unwrap_or_default(),
            game_indices,
            generation_ids,
            fling_power: entry.fling_power,
            fling_effect_id: entry.fling_effect_id,
            names: data.names_map.get(entry.id),
            short_effects: data.short_effects_map.get(entry.id),
            effects: data.effects_map.get(entry.id),
            flavor_texts: data.flavor_texts_map.get(entry.id),
        })
    }
}

#[derive(Debug)]
pub struct ItemConversionData {
    pub names_map: LocalizedValuesMap,
    pub effects_map: LocalizedValuesMap,
    pub short_effects_map: LocalizedValuesMap,
    pub flavor_texts_map: VersionGroupedLocalizedValuesMap,
    pub flag_ids_map: HashMap<i32, Vec<i32>>,
    pub game_indices_map: HashMap<i32, Vec<i32>>,
    pub generation_ids_map: HashMap<i32, Vec<i32>>,
}

impl ItemConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: ItemNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            effects_map: ItemEffectsCSV::load(data_path).unwrap().into_localized_values_map(),
            short_effects_map: ItemShortEffectsCSV::load(data_path).unwrap().into_localized_values_map(),
            flavor_texts_map: ItemFlavorTextCSV::load(data_path).unwrap().into_version_grouped_localized_values_map(),
            flag_ids_map: ItemFlagMapCSV::load(data_path).unwrap().group_by_id(),
            game_indices_map: ItemGameIndicesCSV::load(data_path).unwrap().group_by_id(),
            generation_ids_map: ItemGenerationIdsCSV::load(data_path).unwrap().group_by_id(),
        }
    }
}
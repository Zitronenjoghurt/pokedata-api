use crate::csv::item_flag_prose::{ItemFlagDescriptionsCSV, ItemFlagNamesCSV};
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::item_flag::ItemFlag;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemFlagsCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for ItemFlagsCSV {
    fn file_name() -> &'static str {
        "item_flags"
    }
}

impl ApiCSVEntity for ItemFlagsCSV {
    type ApiType = ItemFlag;
    type ConversionData = ItemFlagConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(ItemFlag {
            id: entry.id,
            identifier: entry.identifier,
            names: data.names_map.get(entry.id),
            descriptions: data.descriptions_map.get(entry.id),
        })
    }
}

#[derive(Debug)]
pub struct ItemFlagConversionData {
    pub names_map: LocalizedValuesMap,
    pub descriptions_map: LocalizedValuesMap,
}

impl ItemFlagConversionData {
    pub fn load(data_path: &PathBuf) -> Self {
        Self {
            names_map: ItemFlagNamesCSV::load(data_path).unwrap().into_localized_values_map(),
            descriptions_map: ItemFlagDescriptionsCSV::load(data_path).unwrap().into_localized_values_map(),
        }
    }
}
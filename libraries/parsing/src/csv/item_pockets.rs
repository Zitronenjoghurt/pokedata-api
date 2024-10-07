use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::item_pocket::ItemPocket;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemPocketsCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for ItemPocketsCSV {
    fn file_name() -> &'static str {
        "item_pockets"
    }
}

impl ApiCSVEntity for ItemPocketsCSV {
    type ApiType = ItemPocket;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(ItemPocket {
            id: entry.id,
            identifier: entry.identifier,
            names: data.get(entry.id),
        })
    }
}
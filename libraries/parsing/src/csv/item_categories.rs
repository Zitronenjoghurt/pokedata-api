use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::item_category::ItemCategory;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemCategoriesCSV {
    pub id: i32,
    pub identifier: String,
    pub pocket_id: i32,
}

impl CSVEntity for ItemCategoriesCSV {
    fn file_name() -> &'static str {
        "item_categories"
    }
}

impl ApiCSVEntity for ItemCategoriesCSV {
    type ApiType = ItemCategory;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(ItemCategory {
            id: entry.id,
            pocket_id: entry.pocket_id,
            identifier: entry.identifier,
            names: data.get(entry.id),
        })
    }
}
use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::egg_group::EggGroup;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EggGroupsCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for EggGroupsCSV {
    fn file_name() -> &'static str {
        "egg_groups"
    }
}

impl ApiCSVEntity for EggGroupsCSV {
    type ApiType = EggGroup;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(EggGroup {
            id: entry.id,
            identifier: entry.identifier,
            names: data.get(entry.id),
        })
    }
}
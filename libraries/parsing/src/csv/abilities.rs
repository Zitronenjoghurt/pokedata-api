use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::ability::Ability;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilitiesCSV {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
    pub is_main_series: u32,
}

impl CSVEntity for AbilitiesCSV {
    fn file_name() -> &'static str {
        "abilities"
    }
}

impl ApiCSVEntity for AbilitiesCSV {
    type ApiType = Ability;
    type ConversionData = ();

    fn convert(entry: Self, _data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(Ability {
            id: entry.id,
            is_main_series: entry.is_main_series == 1,
        })
    }
}
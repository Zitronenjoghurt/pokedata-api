use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::berry_firmness::BerryFirmness;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BerryFirmnessCSV {
    pub id: i32,
    pub identifier: String,
}

impl CSVEntity for BerryFirmnessCSV {
    fn file_name() -> &'static str {
        "berry_firmness"
    }
}

impl ApiCSVEntity for BerryFirmnessCSV {
    type ApiType = BerryFirmness;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(BerryFirmness {
            id: entry.id,
            identifier: entry.identifier,
            names: data.get(entry.id),
        })
    }
}
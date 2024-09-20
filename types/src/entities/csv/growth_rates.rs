use crate::entities::api::growth_rate::GrowthRate;
use crate::entities::api::localized_values::LocalizedValuesMap;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GrowthRatesCSV {
    pub id: u32,
    pub identifier: String,
    pub formula: String,
}

impl CSVEntity for GrowthRatesCSV {
    fn file_name() -> &'static str {
        "growth_rates"
    }
}

impl ApiCSVEntity for GrowthRatesCSV {
    type ApiType = GrowthRate;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            GrowthRate {
                id: entry.id,
                identifier: entry.identifier,
                formula: entry.formula,
                names: data.get(entry.id),
            }
        )
    }
}
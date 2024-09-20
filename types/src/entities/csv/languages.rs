use crate::entities::api::language::Language;
use crate::entities::api::localized_values::LocalizedValuesMap;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LanguagesCSV {
    pub id: u32,
    pub iso639: String,
    pub iso3166: String,
    pub identifier: String,
    pub official: u32,
    pub order: u32,
}

impl CSVEntity for LanguagesCSV {
    fn file_name() -> &'static str {
        "languages"
    }
}

impl ApiCSVEntity for LanguagesCSV {
    type ApiType = Language;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            Language {
                id: entry.id,
                iso639: entry.iso639,
                iso3166: entry.iso3166,
                official: entry.official == 1,
                order: entry.order,
                names: data.get(entry.id),
            }
        )
    }
}
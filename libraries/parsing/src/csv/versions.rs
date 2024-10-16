use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::localized_values::LocalizedValuesMap;
use pokedata_api_entities::api::version::Version;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionsCSV {
    pub id: i32,
    pub version_group_id: i32,
    pub identifier: String,
}

impl CSVEntity for VersionsCSV {
    fn file_name() -> &'static str {
        "versions"
    }
}

impl ApiCSVEntity for VersionsCSV {
    type ApiType = Version;
    type ConversionData = LocalizedValuesMap;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        Ok(
            Version {
                id: entry.id,
                version_group_id: entry.version_group_id,
                identifier: entry.identifier,
                names: data.get(entry.id),
            }
        )
    }
}
use crate::entities::api::localized_values::LocalizedValuesMap;
use crate::entities::api::version::Version;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::api_csv_entity::ApiCSVEntity;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionsCSV {
    pub id: u32,
    pub version_group_id: u32,
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
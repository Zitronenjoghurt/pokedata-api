use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionGroupRegionsCSV {
    pub version_group_id: u32,
    pub region_id: u32,
}

impl CSVEntity for VersionGroupRegionsCSV {
    fn file_name() -> &'static str {
        "version_group_regions"
    }
}

impl IdValuePairing for VersionGroupRegionsCSV {
    type Id = u32;
    type Value = u32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.version_group_id, self.region_id)
    }
}
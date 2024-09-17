use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionGroupsCSV {
    pub id: Option<u32>,
    pub identifier: Option<String>,
    pub generation_id: Option<u32>,
    pub order: Option<u32>,
}

impl CSVEntity for VersionGroupsCSV {
    fn file_name() -> &'static str {
        "version_groups"
    }
}
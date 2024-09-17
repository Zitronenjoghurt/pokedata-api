use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionsCSV {
    pub id: Option<u32>,
    pub version_group_id: Option<u32>,
    pub identifier: Option<String>,
}

impl CSVEntity for VersionsCSV {
    fn file_name() -> &'static str {
        "versions"
    }
}
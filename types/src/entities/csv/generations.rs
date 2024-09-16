use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenerationsCSV {
    pub id: Option<u32>,
    pub main_region_id: Option<u32>,
    pub identifier: Option<String>,
}

impl CSVEntity for GenerationsCSV {
    fn file_name() -> &'static str {
        "generations"
    }
}
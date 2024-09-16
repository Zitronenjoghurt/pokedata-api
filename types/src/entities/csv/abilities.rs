use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilitiesCSV {
    pub id: Option<u32>,
    pub identifier: Option<String>,
    pub generation_id: Option<u32>,
    pub is_main_series: Option<u8>,
}

impl CSVEntity for AbilitiesCSV {
    fn file_name() -> &'static str {
        "abilities"
    }
}
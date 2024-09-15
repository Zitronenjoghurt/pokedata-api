use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Abilities {
    pub id: u32,
    pub identifier: String,
    pub generation_id: u32,
    pub is_main_series: u8,
}

impl CSVEntity for Abilities {
    fn get_file_name(&self) -> &'static str {
        "abilities"
    }
}
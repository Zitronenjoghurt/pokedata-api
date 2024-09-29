use crate::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TypeEfficacyPastCSV {
    pub damage_type_id: u32,
    pub target_type_id: u32,
    pub damage_factor: u32,
    pub generation_id: u32,
}

impl CSVEntity for TypeEfficacyPastCSV {
    fn file_name() -> &'static str {
        "type_efficacy_past"
    }
}
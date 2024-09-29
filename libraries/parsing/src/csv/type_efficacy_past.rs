use crate::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TypeEfficacyPastCSV {
    pub damage_type_id: i32,
    pub target_type_id: i32,
    pub damage_factor: i32,
    pub generation_id: i32,
}

impl CSVEntity for TypeEfficacyPastCSV {
    fn file_name() -> &'static str {
        "type_efficacy_past"
    }
}
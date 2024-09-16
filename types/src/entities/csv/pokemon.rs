use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonCSV {
    pub id: Option<u32>,
    pub identifier: Option<String>,
    pub species_id: Option<u32>,
    pub height: Option<u32>,
    pub weight: Option<u32>,
    pub base_experience: Option<u32>,
    pub order: Option<u32>,
    pub is_default: Option<u32>,
}

impl CSVEntity for PokemonCSV {
    fn file_name() -> &'static str {
        "pokemon"
    }
}
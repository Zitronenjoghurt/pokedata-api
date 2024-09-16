use crate::entities::csv_entity::CSVEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct PokemonHabitatsCSV {
    pub id: Option<u32>,
    pub identifier: Option<String>,
}

impl CSVEntity for PokemonHabitatsCSV {
    fn file_name() -> &'static str {
        "pokemon_habitats"
    }
}
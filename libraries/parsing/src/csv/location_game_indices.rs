use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationGenerationIdsCSV {
    pub location_id: i32,
    pub generation_id: i32,
}

impl CSVEntity for LocationGenerationIdsCSV {
    fn file_name() -> &'static str {
        "location_game_indices"
    }
}

impl IdValuePairing for LocationGenerationIdsCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.location_id, self.generation_id)
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationGameIndicesCSV {
    pub location_id: i32,
    pub game_index: i32,
}

impl CSVEntity for LocationGameIndicesCSV {
    fn file_name() -> &'static str {
        "location_game_indices"
    }
}

impl IdValuePairing for LocationGameIndicesCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.location_id, self.game_index)
    }
}
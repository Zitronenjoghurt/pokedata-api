use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemGenerationIdsCSV {
    pub item_id: i32,
    pub generation_id: i32,
}

impl CSVEntity for ItemGenerationIdsCSV {
    fn file_name() -> &'static str {
        "item_game_indices"
    }
}

impl IdValuePairing for ItemGenerationIdsCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.item_id, self.generation_id)
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemGameIndicesCSV {
    pub item_id: i32,
    pub game_index: i32,
}

impl CSVEntity for ItemGameIndicesCSV {
    fn file_name() -> &'static str {
        "item_game_indices"
    }
}

impl IdValuePairing for ItemGameIndicesCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.item_id, self.game_index)
    }
}
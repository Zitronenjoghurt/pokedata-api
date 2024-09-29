use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveFlagMapCSV {
    pub move_id: i32,
    pub move_flag_id: i32,
}

impl CSVEntity for MoveFlagMapCSV {
    fn file_name() -> &'static str {
        "move_flag_map"
    }
}

impl IdValuePairing for MoveFlagMapCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.move_id, self.move_flag_id)
    }
}
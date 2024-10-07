use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemFlagMapCSV {
    pub item_id: i32,
    pub item_flag_id: i32,
}

impl CSVEntity for ItemFlagMapCSV {
    fn file_name() -> &'static str {
        "item_flag_map"
    }
}

impl IdValuePairing for ItemFlagMapCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.item_id, self.item_flag_id)
    }
}
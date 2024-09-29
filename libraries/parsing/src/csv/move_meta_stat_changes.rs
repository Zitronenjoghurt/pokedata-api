use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveMetaStatChangesCSV {
    pub move_id: i32,
    pub stat_id: i32,
    pub change: i32,
}

impl CSVEntity for MoveMetaStatChangesCSV {
    fn file_name() -> &'static str {
        "move_meta_stat_changes"
    }
}

impl IdValuePairingMapped for MoveMetaStatChangesCSV {
    type Id = i32;
    type Key = i32;
    type Value = i32;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        (self.move_id, self.stat_id, self.change)
    }
}
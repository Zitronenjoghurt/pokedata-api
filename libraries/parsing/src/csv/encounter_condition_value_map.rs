use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing::IdValuePairing;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterConditionValueMapCSV {
    pub encounter_id: i32,
    pub encounter_condition_value_id: i32,
}

impl CSVEntity for EncounterConditionValueMapCSV {
    fn file_name() -> &'static str {
        "encounter_condition_value_map"
    }
}

impl IdValuePairing for EncounterConditionValueMapCSV {
    type Id = i32;
    type Value = i32;

    fn into_pair(self) -> (Self::Id, Self::Value) {
        (self.encounter_id, self.encounter_condition_value_id)
    }
}
use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterConditionProseCSV {
    pub encounter_condition_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for EncounterConditionProseCSV {
    fn file_name() -> &'static str {
        "encounter_condition_prose"
    }
}

impl HasLocalizedValues for EncounterConditionProseCSV {
    fn id(&self) -> i32 {
        self.encounter_condition_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
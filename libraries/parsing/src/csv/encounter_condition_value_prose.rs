use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterConditionValueProseCSV {
    pub encounter_condition_value_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for EncounterConditionValueProseCSV {
    fn file_name() -> &'static str {
        "encounter_condition_value_prose"
    }
}

impl HasLocalizedValues for EncounterConditionValueProseCSV {
    fn id(&self) -> i32 {
        self.encounter_condition_value_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EncounterMethodProseCSV {
    pub encounter_method_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for EncounterMethodProseCSV {
    fn file_name() -> &'static str {
        "encounter_method_prose"
    }
}

impl HasLocalizedValues for EncounterMethodProseCSV {
    fn id(&self) -> i32 {
        self.encounter_method_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
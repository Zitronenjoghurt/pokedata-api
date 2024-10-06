use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EvolutionTriggerProseCSV {
    pub evolution_trigger_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for EvolutionTriggerProseCSV {
    fn file_name() -> &'static str {
        "evolution_trigger_prose"
    }
}

impl HasLocalizedValues for EvolutionTriggerProseCSV {
    fn id(&self) -> i32 {
        self.evolution_trigger_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
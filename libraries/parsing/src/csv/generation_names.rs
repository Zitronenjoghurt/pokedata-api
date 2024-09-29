use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GenerationNamesCSV {
    pub generation_id: u32,
    pub local_language_id: u32,
    pub name: String,
}

impl CSVEntity for GenerationNamesCSV {
    fn file_name() -> &'static str {
        "generation_names"
    }
}

impl HasLocalizedValues for GenerationNamesCSV {
    fn id(&self) -> u32 {
        self.generation_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
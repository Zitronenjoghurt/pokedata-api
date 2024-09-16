use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_localized_names::HasLocalizedNames;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LanguageNamesCSV {
    pub language_id: u32,
    pub local_language_id: u32,
    pub name: String,
}

impl CSVEntity for LanguageNamesCSV {
    fn file_name() -> &'static str {
        "language_names"
    }
}

impl HasLocalizedNames for LanguageNamesCSV {
    fn id(&self) -> u32 {
        self.language_id
    }

    fn language_id(&self) -> u32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
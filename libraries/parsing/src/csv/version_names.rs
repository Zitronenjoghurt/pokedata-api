use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct VersionNamesCSV {
    pub version_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for VersionNamesCSV {
    fn file_name() -> &'static str {
        "version_names"
    }
}

impl HasLocalizedValues for VersionNamesCSV {
    fn id(&self) -> i32 {
        self.version_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
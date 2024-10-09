use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationAreaProseCSV {
    pub location_area_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for LocationAreaProseCSV {
    fn file_name() -> &'static str {
        "location_area_prose"
    }
}

impl HasLocalizedValues for LocationAreaProseCSV {
    fn id(&self) -> i32 {
        self.location_area_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
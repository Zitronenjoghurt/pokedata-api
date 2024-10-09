use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationNamesCSV {
    pub location_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for LocationNamesCSV {
    fn file_name() -> &'static str {
        "location_names"
    }
}

impl HasLocalizedValues for LocationNamesCSV {
    fn id(&self) -> i32 {
        self.location_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct LocationSubtitlesCSV {
    pub location_id: i32,
    pub local_language_id: i32,
    pub subtitle: String,
}

impl CSVEntity for LocationSubtitlesCSV {
    fn file_name() -> &'static str {
        "location_names"
    }
}

impl HasLocalizedValues for LocationSubtitlesCSV {
    fn id(&self) -> i32 {
        self.location_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.subtitle.clone()
    }
}
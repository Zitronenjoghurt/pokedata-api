use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BerryFirmnessNamesCSV {
    pub berry_firmness_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for BerryFirmnessNamesCSV {
    fn file_name() -> &'static str {
        "berry_firmness_names"
    }
}

impl HasLocalizedValues for BerryFirmnessNamesCSV {
    fn id(&self) -> i32 {
        self.berry_firmness_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
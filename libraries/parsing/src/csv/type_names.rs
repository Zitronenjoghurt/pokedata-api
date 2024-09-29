use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TypeNamesCSV {
    pub type_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for TypeNamesCSV {
    fn file_name() -> &'static str {
        "type_names"
    }
}

impl HasLocalizedValues for TypeNamesCSV {
    fn id(&self) -> i32 {
        self.type_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
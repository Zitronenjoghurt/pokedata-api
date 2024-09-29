use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveMetaCategoryProseCSV {
    pub move_meta_category_id: i32,
    pub local_language_id: i32,
    pub description: String,
}

impl CSVEntity for MoveMetaCategoryProseCSV {
    fn file_name() -> &'static str {
        "move_meta_category_prose"
    }
}

impl HasLocalizedValues for MoveMetaCategoryProseCSV {
    fn id(&self) -> i32 {
        self.move_meta_category_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.description.clone()
    }
}
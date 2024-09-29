use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveMetaAilmentNamesCSV {
    pub move_meta_ailment_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for MoveMetaAilmentNamesCSV {
    fn file_name() -> &'static str {
        "move_meta_ailment_names"
    }
}

impl HasLocalizedValues for MoveMetaAilmentNamesCSV {
    fn id(&self) -> i32 {
        self.move_meta_ailment_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
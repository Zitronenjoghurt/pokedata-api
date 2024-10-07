use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemPocketNamesCSV {
    pub item_pocket_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for ItemPocketNamesCSV {
    fn file_name() -> &'static str {
        "item_pocket_names"
    }
}

impl HasLocalizedValues for ItemPocketNamesCSV {
    fn id(&self) -> i32 {
        self.item_pocket_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}
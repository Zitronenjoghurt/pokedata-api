use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use crate::traits::has_version_group_id::HasVersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemFlavorTextCSV {
    pub item_id: i32,
    pub version_group_id: i32,
    pub language_id: i32,
    pub flavor_text: String,
}

impl CSVEntity for ItemFlavorTextCSV {
    fn file_name() -> &'static str {
        "item_flavor_text"
    }
}

impl HasVersionGroupId for ItemFlavorTextCSV {
    fn version_group_id(&self) -> i32 {
        self.version_group_id
    }
}

impl HasLocalizedValues for ItemFlavorTextCSV {
    fn id(&self) -> i32 {
        self.item_id
    }

    fn language_id(&self) -> i32 {
        self.language_id
    }

    fn name(&self) -> String {
        self.flavor_text.clone()
    }
}
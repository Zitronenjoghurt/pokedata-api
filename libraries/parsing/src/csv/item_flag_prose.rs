use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemFlagNamesCSV {
    pub item_flag_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for ItemFlagNamesCSV {
    fn file_name() -> &'static str {
        "item_flag_prose"
    }
}

impl HasLocalizedValues for ItemFlagNamesCSV {
    fn id(&self) -> i32 {
        self.item_flag_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemFlagDescriptionsCSV {
    pub item_flag_id: i32,
    pub local_language_id: i32,
    pub description: String,
}

impl CSVEntity for ItemFlagDescriptionsCSV {
    fn file_name() -> &'static str {
        "item_flag_prose"
    }
}

impl HasLocalizedValues for ItemFlagDescriptionsCSV {
    fn id(&self) -> i32 {
        self.item_flag_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.description.clone()
    }
}
use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveFlagNamesCSV {
    pub move_flag_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for MoveFlagNamesCSV {
    fn file_name() -> &'static str {
        "move_flag_prose"
    }
}

impl HasLocalizedValues for MoveFlagNamesCSV {
    fn id(&self) -> i32 {
        self.move_flag_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveFlagDescriptionsCSV {
    pub move_flag_id: i32,
    pub local_language_id: i32,
    pub description: String,
}

impl CSVEntity for MoveFlagDescriptionsCSV {
    fn file_name() -> &'static str {
        "move_flag_prose"
    }
}

impl HasLocalizedValues for MoveFlagDescriptionsCSV {
    fn id(&self) -> i32 {
        self.move_flag_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.description.clone()
    }
}
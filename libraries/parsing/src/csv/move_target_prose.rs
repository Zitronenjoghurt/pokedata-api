use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveTargetNamesCSV {
    pub move_target_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for MoveTargetNamesCSV {
    fn file_name() -> &'static str {
        "move_target_prose"
    }
}

impl HasLocalizedValues for MoveTargetNamesCSV {
    fn id(&self) -> i32 {
        self.move_target_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveTargetDescriptionsCSV {
    pub move_target_id: i32,
    pub local_language_id: i32,
    pub description: String,
}

impl CSVEntity for MoveTargetDescriptionsCSV {
    fn file_name() -> &'static str {
        "move_target_prose"
    }
}

impl HasLocalizedValues for MoveTargetDescriptionsCSV {
    fn id(&self) -> i32 {
        self.move_target_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.description.clone()
    }
}
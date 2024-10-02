use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveDamageClassNamesCSV {
    pub move_damage_class_id: i32,
    pub local_language_id: i32,
    pub name: String,
}

impl CSVEntity for MoveDamageClassNamesCSV {
    fn file_name() -> &'static str {
        "move_damage_class_prose"
    }
}

impl HasLocalizedValues for MoveDamageClassNamesCSV {
    fn id(&self) -> i32 {
        self.move_damage_class_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.name.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveDamageClassDescriptionsCSV {
    pub move_damage_class_id: i32,
    pub local_language_id: i32,
    pub description: String,
}

impl CSVEntity for MoveDamageClassDescriptionsCSV {
    fn file_name() -> &'static str {
        "move_damage_class_prose"
    }
}

impl HasLocalizedValues for MoveDamageClassDescriptionsCSV {
    fn id(&self) -> i32 {
        self.move_damage_class_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.description.clone()
    }
}
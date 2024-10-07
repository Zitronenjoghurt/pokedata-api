use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemShortEffectsCSV {
    pub item_id: i32,
    pub local_language_id: i32,
    pub short_effect: String,
}

impl CSVEntity for ItemShortEffectsCSV {
    fn file_name() -> &'static str {
        "item_prose"
    }
}

impl HasLocalizedValues for ItemShortEffectsCSV {
    fn id(&self) -> i32 {
        self.item_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.short_effect.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct ItemEffectsCSV {
    pub item_id: i32,
    pub local_language_id: i32,
    pub effect: String,
}

impl CSVEntity for ItemEffectsCSV {
    fn file_name() -> &'static str {
        "item_prose"
    }
}

impl HasLocalizedValues for ItemEffectsCSV {
    fn id(&self) -> i32 {
        self.item_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.effect.clone()
    }
}
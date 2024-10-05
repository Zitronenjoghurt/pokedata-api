use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilityShortEffectsCSV {
    pub ability_id: i32,
    pub local_language_id: i32,
    pub short_effect: String,
}

impl CSVEntity for AbilityShortEffectsCSV {
    fn file_name() -> &'static str {
        "ability_prose"
    }
}

impl HasLocalizedValues for AbilityShortEffectsCSV {
    fn id(&self) -> i32 {
        self.ability_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.short_effect.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilityEffectsCSV {
    pub ability_id: i32,
    pub local_language_id: i32,
    pub effect: String,
}

impl CSVEntity for AbilityEffectsCSV {
    fn file_name() -> &'static str {
        "ability_prose"
    }
}

impl HasLocalizedValues for AbilityEffectsCSV {
    fn id(&self) -> i32 {
        self.ability_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.effect.clone()
    }
}
use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveEffectShortEffectsCSV {
    pub move_effect_id: i32,
    pub local_language_id: i32,
    pub short_effect: String,
}

impl CSVEntity for MoveEffectShortEffectsCSV {
    fn file_name() -> &'static str {
        "move_effect_prose"
    }
}

impl HasLocalizedValues for MoveEffectShortEffectsCSV {
    fn id(&self) -> i32 {
        self.move_effect_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.short_effect.clone()
    }
}


#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveEffectEffectsCSV {
    pub move_effect_id: i32,
    pub local_language_id: i32,
    pub effect: String,
}

impl CSVEntity for MoveEffectEffectsCSV {
    fn file_name() -> &'static str {
        "move_effect_prose"
    }
}

impl HasLocalizedValues for MoveEffectEffectsCSV {
    fn id(&self) -> i32 {
        self.move_effect_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.effect.clone()
    }
}
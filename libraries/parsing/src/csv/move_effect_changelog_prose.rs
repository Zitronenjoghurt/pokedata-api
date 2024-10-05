use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveEffectChangelogProseCSV {
    pub move_effect_changelog_id: i32,
    pub local_language_id: i32,
    pub effect: String,
}

impl CSVEntity for MoveEffectChangelogProseCSV {
    fn file_name() -> &'static str {
        "move_effect_changelog_prose"
    }
}

impl HasLocalizedValues for MoveEffectChangelogProseCSV {
    fn id(&self) -> i32 {
        self.move_effect_changelog_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.effect.clone()
    }
}
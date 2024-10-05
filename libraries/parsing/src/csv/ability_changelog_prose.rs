use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilityChangelogProseCSV {
    pub ability_changelog_id: i32,
    pub local_language_id: i32,
    pub effect: String,
}

impl CSVEntity for AbilityChangelogProseCSV {
    fn file_name() -> &'static str {
        "ability_changelog_prose"
    }
}

impl HasLocalizedValues for AbilityChangelogProseCSV {
    fn id(&self) -> i32 {
        self.ability_changelog_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.effect.clone()
    }
}
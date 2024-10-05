use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use pokedata_api_entities::api::ability_changelog_entry::AbilityChangelogEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct AbilityChangelogCSV {
    pub id: i32,
    pub ability_id: i32,
    pub changed_in_version_group_id: i32,
}

impl CSVEntity for AbilityChangelogCSV {
    fn file_name() -> &'static str {
        "ability_changelog"
    }
}

impl IdValuePairingMapped for AbilityChangelogCSV {
    type Id = i32;
    type Key = i32;
    type Value = AbilityChangelogEntry;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        let value = AbilityChangelogEntry {
            id: self.id,
            effects: None,
        };
        (self.ability_id, self.changed_in_version_group_id, value)
    }
}
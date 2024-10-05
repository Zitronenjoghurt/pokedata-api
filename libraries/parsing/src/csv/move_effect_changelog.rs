use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use pokedata_api_entities::api::pokemon_move_effect_changelog_entry::PokemonMoveEffectChangelogEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveEffectChangelogCSV {
    pub id: i32,
    pub effect_id: i32,
    pub changed_in_version_group_id: i32,
}

impl CSVEntity for MoveEffectChangelogCSV {
    fn file_name() -> &'static str {
        "move_effect_changelog"
    }
}

impl IdValuePairingMapped for MoveEffectChangelogCSV {
    type Id = i32;
    type Key = i32;
    type Value = PokemonMoveEffectChangelogEntry;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        let value = PokemonMoveEffectChangelogEntry {
            id: self.id,
            effects: None,
        };
        (self.effect_id, self.changed_in_version_group_id, value)
    }
}
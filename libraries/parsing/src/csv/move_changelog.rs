use crate::csv_entity::CSVEntity;
use crate::traits::id_value_pairing_mapped::IdValuePairingMapped;
use pokedata_api_entities::api::pokemon_move_changelog_entry::PokemonMoveChangelogEntry;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveChangelogCSV {
    pub move_id: i32,
    pub changed_in_version_group_id: i32,
    pub type_id: Option<i32>,
    pub power: Option<i32>,
    pub pp: Option<i32>,
    pub accuracy: Option<i32>,
    pub priority: Option<i32>,
    pub target_id: Option<i32>,
    pub effect_id: Option<i32>,
    pub effect_chance: Option<i32>,
}

impl CSVEntity for MoveChangelogCSV {
    fn file_name() -> &'static str {
        "move_changelog"
    }
}

impl IdValuePairingMapped for MoveChangelogCSV {
    type Id = i32;
    type Key = i32;
    type Value = PokemonMoveChangelogEntry;

    fn into_triple(self) -> (Self::Id, Self::Key, Self::Value) {
        let value = PokemonMoveChangelogEntry {
            type_id: self.type_id,
            power: self.power,
            pp: self.pp,
            accuracy: self.accuracy,
            priority: self.priority,
            target_id: self.target_id,
            effect_id: self.effect_id,
            effect_chance: self.effect_chance,
        };
        (self.move_id, self.changed_in_version_group_id, value)
    }
}
use crate::api::localized_values::LocalizedValues;
use crate::api::pokemon_move_effect_changelog_entry::PokemonMoveEffectChangelogEntry;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonMoveEffect {
    pub id: i32,
    pub effect: Option<LocalizedValues>,
    pub short_effect: Option<LocalizedValues>,
    /// Changelog entries mapped by the version group id they were introduced in
    pub changelogs: HashMap<i32, PokemonMoveEffectChangelogEntry>,
    /// IDs of the moves that have this effect
    pub move_ids: Vec<i32>,
}

impl HasId for PokemonMoveEffect {
    fn id(&self) -> i32 {
        self.id
    }
}
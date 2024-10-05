use crate::api::localized_values::LocalizedValues;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema, PartialEq, Eq)]
pub struct PokemonMoveEffectChangelogEntry {
    pub id: i32,
    pub effects: Option<LocalizedValues>,
}
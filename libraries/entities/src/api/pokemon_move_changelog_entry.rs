use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonMoveChangelogEntry {
    pub type_id: Option<i32>,
    pub power: Option<i32>,
    pub pp: Option<i32>,
    pub accuracy: Option<i32>,
    pub priority: Option<i32>,
    pub target_id: Option<i32>,
    pub effect_id: Option<i32>,
    pub effect_chance: Option<i32>,
}
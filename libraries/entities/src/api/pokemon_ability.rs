use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonAbility {
    pub ability_id: i32,
    pub is_hidden: bool,
    pub slot: i32,
}
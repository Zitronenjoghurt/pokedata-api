use crate::api::pokemon_stats::PokemonStats;
use crate::api::sprites::SpritePaths;
use crate::api::type_slots::{TypeSlots, TypeSlotsPast};
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Pokemon {
    pub id: i32,
    pub identifier: String,
    /// The species this Pokémon belongs to.
    pub species_id: i32,
    pub height_decimeter: i32,
    pub weight_hectograms: i32,
    /// The base experience gained for defeating this Pokémon.
    pub base_experience: i32,
    /// Order for sorting. Almost national order, except families are grouped together.
    pub dex_order: i32,
    /// Set for exactly one Pokémon used as the default for each species.
    pub is_default: bool,
    pub stats: PokemonStats,
    pub types: TypeSlots,
    pub types_past: Option<TypeSlotsPast>,
    /// The version ids of the games this pokemon exists in
    pub version_ids: Vec<i32>,
    /// Sprites mapped by sprite groups and sprite types
    pub sprites: SpritePaths,
    /// Pokemon SpritePaths mapped by their form identifier
    pub form_sprites: HashMap<String, SpritePaths>,
}

impl HasId for Pokemon {
    fn id(&self) -> i32 {
        self.id
    }
}
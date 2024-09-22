use crate::entities::api::pokemon_stats::PokemonStats;
use crate::entities::api::type_slots::{TypeSlots, TypeSlotsPast};
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Pokemon {
    pub id: u32,
    pub identifier: String,
    /// The species this Pokémon belongs to.
    pub species_id: u32,
    pub height_decimeter: u32,
    pub weight_hectograms: u32,
    /// The base experience gained for defeating this Pokémon.
    pub base_experience: u32,
    /// Order for sorting. Almost national order, except families are grouped together.
    pub dex_order: u32,
    /// Set for exactly one Pokémon used as the default for each species.
    pub is_default: bool,
    pub stats: PokemonStats,
    pub types: TypeSlots,
    pub types_past: Option<TypeSlotsPast>,
    /// The version ids of the games this pokemon exists in
    pub version_ids: Vec<u32>,
}

impl HasId for Pokemon {
    fn id(&self) -> u32 {
        self.id
    }
}
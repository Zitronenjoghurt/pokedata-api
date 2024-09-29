use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct EvolutionChainNode {
    pub species_id: u32,
    /// ID of the evolution containing the information how to evolve to this species
    pub evolution_id: Option<u32>,
    pub next: Vec<EvolutionChainNode>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct EvolutionChain {
    pub id: u32,
    pub baby_trigger_item_id: Option<u32>,
    pub chain: EvolutionChainNode,
}

impl HasId for EvolutionChain {
    fn id(&self) -> u32 {
        self.id
    }
}
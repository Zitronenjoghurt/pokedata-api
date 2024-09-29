use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct EvolutionChainNode {
    pub species_id: i32,
    /// ID of the evolution containing the information how to evolve to this species
    pub evolution_id: Option<i32>,
    pub next: Vec<EvolutionChainNode>,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct EvolutionChain {
    pub id: i32,
    pub baby_trigger_item_id: Option<i32>,
    pub chain: EvolutionChainNode,
}

impl HasId for EvolutionChain {
    fn id(&self) -> i32 {
        self.id
    }
}
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct TotalStats {
    pub pokemon_tcg_card_count: usize,
    pub trainer_tcg_card_count: usize,
    pub energy_tcg_card_count: usize,
}
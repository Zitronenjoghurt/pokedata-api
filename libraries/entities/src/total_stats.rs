use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TotalStats {
    pub tcg_cards_count: usize,
    pub tcg_sets_count: usize,
}
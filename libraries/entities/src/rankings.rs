use crate::data_structures::ranking::Ranking;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rankings {
    pub species_tcg_card_count: Ranking<i32, usize>,
    pub species_tcg_set_count: Ranking<i32, usize>,
}
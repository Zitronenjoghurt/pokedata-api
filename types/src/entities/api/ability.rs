use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub generation_id: u32,
    pub is_main_series: bool,
}
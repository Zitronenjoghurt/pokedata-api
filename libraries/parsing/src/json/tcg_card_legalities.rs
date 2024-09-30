use pokedata_api_entities::api::tcg_card::TcgCardLegalities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgCardLegalitiesJSON {
    pub standard: Option<String>,
    pub expanded: Option<String>,
    pub unlimited: String,
}

impl Into<TcgCardLegalities> for TcgCardLegalitiesJSON {
    fn into(self) -> TcgCardLegalities {
        TcgCardLegalities {
            standard: self.standard,
            expanded: self.expanded,
            unlimited: self.unlimited,
        }
    }
}
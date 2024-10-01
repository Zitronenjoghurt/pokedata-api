use pokedata_api_entities::api::tcg_legalities::TcgLegalities;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgLegalitiesJSON {
    pub standard: Option<String>,
    pub expanded: Option<String>,
    pub unlimited: String,
}

impl Into<TcgLegalities> for TcgLegalitiesJSON {
    fn into(self) -> TcgLegalities {
        TcgLegalities {
            standard: self.standard,
            expanded: self.expanded,
            unlimited: self.unlimited,
        }
    }
}
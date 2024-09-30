use pokedata_api_entities::api::tcg_card::TcgCardAncientTrait;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgCardAncientTraitJSON {
    pub name: String,
    pub text: String,
}

impl Into<TcgCardAncientTrait> for TcgCardAncientTraitJSON {
    fn into(self) -> TcgCardAncientTrait {
        TcgCardAncientTrait {
            name: self.name,
            text: self.text,
        }
    }
}
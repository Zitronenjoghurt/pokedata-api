use pokedata_api_entities::api::tcg_card::TcgCardAbility;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgCardAbilityJSON {
    pub name: String,
    pub text: String,
    #[serde(rename = "type")]
    pub ability_type: String,
}

impl Into<TcgCardAbility> for TcgCardAbilityJSON {
    fn into(self) -> TcgCardAbility {
        TcgCardAbility {
            name: self.name,
            text: self.text,
            ability_type: self.ability_type,
        }
    }
}
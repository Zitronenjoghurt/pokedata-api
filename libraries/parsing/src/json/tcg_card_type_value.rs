use pokedata_api_entities::api::tcg_card::{TcgCardResistance, TcgCardWeakness};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgCardTypeValueJSON {
    #[serde(rename = "type")]
    pub value_type: String,
    pub value: String,
}

impl Into<TcgCardWeakness> for TcgCardTypeValueJSON {
    fn into(self) -> TcgCardWeakness {
        TcgCardWeakness {
            weakness_type: self.value_type,
            value: self.value,
        }
    }
}

impl Into<TcgCardResistance> for TcgCardTypeValueJSON {
    fn into(self) -> TcgCardResistance {
        TcgCardResistance {
            resistance_type: self.value_type,
            value: self.value,
        }
    }
}
use pokedata_api_entities::api::tcg_card::TcgCardAttack;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgCardAttackJSON {
    pub cost: Vec<String>,
    pub name: String,
    pub text: String,
    pub damage: Option<String>,
    pub converted_energy_cost: Option<i32>,
}

impl Into<TcgCardAttack> for TcgCardAttackJSON {
    fn into(self) -> TcgCardAttack {
        TcgCardAttack {
            cost: self.cost,
            name: self.name,
            text: self.text,
            damage: self.damage,
            converted_energy_cost: self.converted_energy_cost,
        }
    }
}
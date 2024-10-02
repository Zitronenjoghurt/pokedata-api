use crate::api::tcg_legalities::TcgLegalities;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgCard {
    pub id: i32,
    pub identifier: String,
    pub name: String,
    pub super_type: String,
    pub sub_types: Vec<String>,
    pub level: Option<String>,
    pub hp: Option<i32>,
    pub types: Vec<String>,
    pub evolves_from: Option<String>,
    pub evolves_to: Vec<String>,
    pub rules: Vec<String>,
    pub ancient_trait: Option<TcgCardAncientTrait>,
    pub abilities: Vec<TcgCardAbility>,
    pub attacks: Vec<TcgCardAttack>,
    pub weaknesses: Vec<TcgCardWeakness>,
    pub resistances: Vec<TcgCardResistance>,
    pub retreat_cost: Vec<String>,
    pub converted_retreat_cost: Option<i32>,
    pub number: String,
    pub artist: Option<String>,
    pub rarity: Option<String>,
    pub flavor_text: Option<String>,
    pub species_ids: Vec<i32>,
    pub legalities: TcgLegalities,
    pub regulation_mark: Option<String>,
    pub images: TcgCardImages,
    pub set_id: Option<i32>,
    pub set_identifier: Option<String>,
}

impl HasId for TcgCard {
    fn id(&self) -> i32 {
        self.id.clone()
    }
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgCardAncientTrait {
    pub name: String,
    pub text: String,
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgCardAbility {
    pub name: String,
    pub text: String,
    #[serde(rename = "type")]
    pub ability_type: String,
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgCardAttack {
    pub cost: Vec<String>,
    pub name: String,
    pub text: String,
    pub damage: Option<String>,
    pub converted_energy_cost: Option<i32>,
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgCardWeakness {
    #[serde(rename = "type")]
    pub weakness_type: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgCardResistance {
    #[serde(rename = "type")]
    pub resistance_type: String,
    pub value: String,
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgCardImages {
    pub small: String,
    pub large: String,
}
use crate::json::tcg_card_ability::TcgCardAbilityJSON;
use crate::json::tcg_card_ancient_trait::TcgCardAncientTraitJSON;
use crate::json::tcg_card_attack::TcgCardAttackJSON;
use crate::json::tcg_card_images::TcgCardImagesJSON;
use crate::json::tcg_card_type_value::TcgCardTypeValueJSON;
use crate::json::tcg_legalities::TcgLegalitiesJSON;
use pokedata_api_entities::api::tcg_card::TcgCard;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgCardJSON {
    pub id: String,
    pub name: String,
    pub supertype: Option<String>,
    pub subtypes: Option<Vec<String>>,
    pub level: Option<String>,
    pub hp: Option<String>,
    pub types: Option<Vec<String>>,
    #[serde(rename = "evolvesFrom")]
    pub evolves_from: Option<String>,
    #[serde(rename = "evolvesTo")]
    pub evolves_to: Option<Vec<String>>,
    pub rules: Option<Vec<String>>,
    #[serde(rename = "ancientTrait")]
    pub ancient_trait: Option<TcgCardAncientTraitJSON>,
    pub abilities: Option<Vec<TcgCardAbilityJSON>>,
    pub attacks: Option<Vec<TcgCardAttackJSON>>,
    pub weaknesses: Option<Vec<TcgCardTypeValueJSON>>,
    pub resistances: Option<Vec<TcgCardTypeValueJSON>>,
    #[serde(rename = "retreatCost")]
    pub retreat_cost: Option<Vec<String>>,
    #[serde(rename = "convertedRetreatCost")]
    pub converted_retreat_cost: Option<i32>,
    // ToDo: Set id
    pub number: String,
    pub artist: Option<String>,
    pub rarity: Option<String>,
    #[serde(rename = "flavorText")]
    pub flavor_text: Option<String>,
    #[serde(rename = "nationalPokedexNumbers")]
    pub national_pokedex_numbers: Option<Vec<i32>>,
    pub legalities: TcgLegalitiesJSON,
    #[serde(rename = "regulationMark")]
    pub regulation_mark: Option<String>,
    pub images: TcgCardImagesJSON,
}

impl TcgCardJSON {
    pub fn convert_to_card(self, index: i32) -> TcgCard {
        let set_identifier = get_set_identifier(&self.id);

        TcgCard {
            id: index,
            identifier: self.id,
            name: self.name,
            super_type: self.supertype.unwrap_or("none".to_string()),
            sub_types: self.subtypes.unwrap_or_default(),
            level: self.level,
            hp: self.hp.and_then(|s| Some(s.parse::<i32>().expect("Unable to parse TcgCard HP value"))),
            types: self.types.unwrap_or_default(),
            evolves_from: self.evolves_from,
            evolves_to: self.evolves_to.unwrap_or_default(),
            rules: self.rules.unwrap_or_default(),
            ancient_trait: self.ancient_trait.map(Into::into),
            abilities: self.abilities.unwrap_or_default().into_iter().map(Into::into).collect(),
            attacks: self.attacks.unwrap_or_default().into_iter().map(Into::into).collect(),
            weaknesses: self.weaknesses.unwrap_or_default().into_iter().map(Into::into).collect(),
            resistances: self.resistances.unwrap_or_default().into_iter().map(Into::into).collect(),
            retreat_cost: self.retreat_cost.unwrap_or_default(),
            converted_retreat_cost: self.converted_retreat_cost,
            number: self.number,
            artist: self.artist,
            rarity: self.rarity,
            flavor_text: self.flavor_text,
            species_ids: self.national_pokedex_numbers.unwrap_or_default(),
            legalities: self.legalities.into(),
            regulation_mark: self.regulation_mark,
            images: self.images.into(),
            set_id: None,
            set_identifier,
        }
    }
}

fn get_set_identifier(card_identifier: &String) -> Option<String> {
    match card_identifier.find('-') {
        Some(index) => Some(card_identifier[..index].to_string()),
        None => None,
    }
}
use crate::entities::api::ability::Ability;
use crate::entities::api::generation::Generation;
use crate::entities::api::growth_rate::GrowthRate;
use crate::entities::api::language::Language;
use crate::entities::api::pokemon::Pokemon;
use crate::entities::api::pokemon_color::PokemonColor;
use crate::entities::api::pokemon_habitat::PokemonHabitat;
use crate::entities::api::pokemon_shape::PokemonShape;
use crate::entities::api::species::Species;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    pub abilities: HashMap<u32, Ability>,
    pub colors: HashMap<u32, PokemonColor>,
    pub generations: HashMap<u32, Generation>,
    pub growth_rates: HashMap<u32, GrowthRate>,
    pub habitats: HashMap<u32, PokemonHabitat>,
    pub languages: HashMap<u32, Language>,
    pub pokemon: HashMap<u32, Pokemon>,
    pub shapes: HashMap<u32, PokemonShape>,
    pub species: HashMap<u32, Species>,
}
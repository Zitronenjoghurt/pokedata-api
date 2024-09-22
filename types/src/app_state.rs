use crate::entities::api::ability::Ability;
use crate::entities::api::evolution::Evolution;
use crate::entities::api::evolution_chain::EvolutionChain;
use crate::entities::api::generation::Generation;
use crate::entities::api::growth_rate::GrowthRate;
use crate::entities::api::language::Language;
use crate::entities::api::pokemon::Pokemon;
use crate::entities::api::pokemon_color::PokemonColor;
use crate::entities::api::pokemon_habitat::PokemonHabitat;
use crate::entities::api::pokemon_shape::PokemonShape;
use crate::entities::api::pokemon_type::PokemonType;
use crate::entities::api::pokemon_type_efficacy::PokemonTypeEfficacies;
use crate::entities::api::region::Region;
use crate::entities::api::species::Species;
use crate::entities::api::stat::Stat;
use crate::entities::api::version::Version;
use crate::entities::api::version_group::VersionGroup;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    pub abilities: HashMap<u32, Ability>,
    pub colors: HashMap<u32, PokemonColor>,
    pub evolutions: HashMap<u32, Evolution>,
    pub evolution_chains: HashMap<u32, EvolutionChain>,
    pub generations: HashMap<u32, Generation>,
    pub growth_rates: HashMap<u32, GrowthRate>,
    pub habitats: HashMap<u32, PokemonHabitat>,
    pub languages: HashMap<u32, Language>,
    pub pokemon: HashMap<u32, Pokemon>,
    pub regions: HashMap<u32, Region>,
    pub shapes: HashMap<u32, PokemonShape>,
    pub species: HashMap<u32, Species>,
    pub stats: HashMap<u32, Stat>,
    pub types: HashMap<u32, PokemonType>,
    pub type_efficacies: PokemonTypeEfficacies,
    pub versions: HashMap<u32, Version>,
    pub version_groups: HashMap<u32, VersionGroup>,
    pub latest_generation: u32,
    pub major_type_ids: Vec<u32>,
}
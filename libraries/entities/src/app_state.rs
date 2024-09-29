use crate::api::ability::Ability;
use crate::api::evolution::Evolution;
use crate::api::evolution_chain::EvolutionChain;
use crate::api::generation::Generation;
use crate::api::growth_rate::GrowthRate;
use crate::api::language::Language;
use crate::api::pokemon::Pokemon;
use crate::api::pokemon_color::PokemonColor;
use crate::api::pokemon_habitat::PokemonHabitat;
use crate::api::pokemon_move::PokemonMove;
use crate::api::pokemon_move_target::PokemonMoveTarget;
use crate::api::pokemon_shape::PokemonShape;
use crate::api::pokemon_type::PokemonType;
use crate::api::pokemon_type_efficacy::PokemonTypeEfficacies;
use crate::api::region::Region;
use crate::api::species::Species;
use crate::api::stat::Stat;
use crate::api::version::Version;
use crate::api::version_group::VersionGroup;
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
    pub moves: HashMap<u32, PokemonMove>,
    pub move_targets: HashMap<u32, PokemonMoveTarget>,
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
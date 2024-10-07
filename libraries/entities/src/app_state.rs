use crate::api::ability::Ability;
use crate::api::berry::Berry;
use crate::api::berry_firmness::BerryFirmness;
use crate::api::berry_flavor::BerryFlavor;
use crate::api::egg_group::EggGroup;
use crate::api::evolution::Evolution;
use crate::api::evolution_chain::EvolutionChain;
use crate::api::evolution_trigger::EvolutionTrigger;
use crate::api::generation::Generation;
use crate::api::growth_rate::GrowthRate;
use crate::api::item::Item;
use crate::api::item_category::ItemCategory;
use crate::api::item_flag::ItemFlag;
use crate::api::item_pocket::ItemPocket;
use crate::api::language::Language;
use crate::api::machine::Machine;
use crate::api::pokemon::Pokemon;
use crate::api::pokemon_color::PokemonColor;
use crate::api::pokemon_habitat::PokemonHabitat;
use crate::api::pokemon_move::PokemonMove;
use crate::api::pokemon_move_ailment::PokemonMoveAilment;
use crate::api::pokemon_move_category::PokemonMoveCategory;
use crate::api::pokemon_move_damage_class::PokemonMoveDamageClass;
use crate::api::pokemon_move_effect::PokemonMoveEffect;
use crate::api::pokemon_move_flag::PokemonMoveFlag;
use crate::api::pokemon_move_target::PokemonMoveTarget;
use crate::api::pokemon_shape::PokemonShape;
use crate::api::pokemon_type::PokemonType;
use crate::api::pokemon_type_efficacy::PokemonTypeEfficacies;
use crate::api::region::Region;
use crate::api::species::Species;
use crate::api::stat::Stat;
use crate::api::tcg_card::TcgCard;
use crate::api::tcg_set::TcgSet;
use crate::api::version::Version;
use crate::api::version_group::VersionGroup;
use crate::search_indices::SearchIndices;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppState {
    pub abilities: HashMap<i32, Ability>,
    pub berries: HashMap<i32, Berry>,
    pub berry_firmness: HashMap<i32, BerryFirmness>,
    pub berry_flavors: HashMap<i32, BerryFlavor>,
    pub colors: HashMap<i32, PokemonColor>,
    pub egg_groups: HashMap<i32, EggGroup>,
    pub evolutions: HashMap<i32, Evolution>,
    pub evolution_chains: HashMap<i32, EvolutionChain>,
    pub evolution_triggers: HashMap<i32, EvolutionTrigger>,
    pub generations: HashMap<i32, Generation>,
    pub growth_rates: HashMap<i32, GrowthRate>,
    pub habitats: HashMap<i32, PokemonHabitat>,
    pub items: HashMap<i32, Item>,
    pub item_categories: HashMap<i32, ItemCategory>,
    pub item_flags: HashMap<i32, ItemFlag>,
    pub item_pockets: HashMap<i32, ItemPocket>,
    pub languages: HashMap<i32, Language>,
    pub machines: HashMap<i32, Machine>,
    pub moves: HashMap<i32, PokemonMove>,
    pub move_ailments: HashMap<i32, PokemonMoveAilment>,
    pub move_categories: HashMap<i32, PokemonMoveCategory>,
    pub move_damage_classes: HashMap<i32, PokemonMoveDamageClass>,
    pub move_effects: HashMap<i32, PokemonMoveEffect>,
    pub move_flags: HashMap<i32, PokemonMoveFlag>,
    pub move_targets: HashMap<i32, PokemonMoveTarget>,
    pub pokemon: HashMap<i32, Pokemon>,
    pub regions: HashMap<i32, Region>,
    pub shapes: HashMap<i32, PokemonShape>,
    pub species: HashMap<i32, Species>,
    pub stats: HashMap<i32, Stat>,
    pub tcg_cards: HashMap<i32, TcgCard>,
    pub tcg_sets: HashMap<i32, TcgSet>,
    pub types: HashMap<i32, PokemonType>,
    pub type_efficacies: PokemonTypeEfficacies,
    pub versions: HashMap<i32, Version>,
    pub version_groups: HashMap<i32, VersionGroup>,
    pub latest_generation: i32,
    pub major_type_ids: Vec<i32>,
    pub search_indices: SearchIndices,
}
use ability::*;
use berry::*;
use berry_firmness::*;
use berry_flavor::*;
use evolution::*;
use evolution_chain::*;
use generation::*;
use growth_rate::*;
use language::*;
use pokedata_api_entities::api::*;
use pokedata_api_entities::traits::has_id::HasId;
use pokemon::*;
use pokemon_color::*;
use pokemon_habitat::*;
use pokemon_move::*;
use pokemon_move_ailment::*;
use pokemon_move_category::*;
use pokemon_move_damage_class::*;
use pokemon_move_effect::*;
use pokemon_move_flag::*;
use pokemon_move_target::*;
use pokemon_shape::*;
use pokemon_type::*;
use region::*;
use serde::{Deserialize, Serialize, Serializer};
use species::*;
use stat::*;
use tcg_card::TcgCard;
use tcg_set::TcgSet;
use utoipa::ToSchema;
use version::*;
use version_group::*;

#[derive(Deserialize, ToSchema)]
#[aliases(
    AbilityBulkResponse = BulkResponse<Ability>,
    BerryBulkResponse = BulkResponse<Berry>,
    BerryFirmnessBulkResponse = BulkResponse<BerryFirmness>,
    BerryFlavorBulkResponse = BulkResponse<BerryFlavor>,
    ColorBulkResponse = BulkResponse<PokemonColor>,
    EvolutionBulkResponse = BulkResponse<Evolution>,
    EvolutionChainBulkResponse = BulkResponse<EvolutionChain>,
    GenerationBulkResponse = BulkResponse<Generation>,
    GrowthRateBulkResponse = BulkResponse<GrowthRate>,
    HabitatBulkResponse = BulkResponse<PokemonHabitat>,
    LanguageBulkResponse = BulkResponse<Language>,
    PokemonBulkResponse = BulkResponse<Pokemon>,
    PokemonMoveBulkResponse = BulkResponse<PokemonMove>,
    PokemonMoveAilmentBulkResponse = BulkResponse<PokemonMoveAilment>,
    PokemonMoveCategoryBulkResponse = BulkResponse<PokemonMoveCategory>,
    PokemonMoveDamageClassBulkResponse = BulkResponse<PokemonMoveDamageClass>,
    PokemonMoveEffectBulkResponse = BulkResponse<PokemonMoveEffect>,
    PokemonMoveFlagBulkResponse = BulkResponse<PokemonMoveFlag>,
    PokemonMoveTargetBulkResponse = BulkResponse<PokemonMoveTarget>,
    PokemonTypeBulkResponse = BulkResponse<PokemonType>,
    RegionBulkResponse = BulkResponse<Region>,
    ShapeBulkResponse = BulkResponse<PokemonShape>,
    SpeciesBulkResponse = BulkResponse<Species>,
    StatBulkResponse = BulkResponse<Stat>,
    TcgCardBulkResponse = BulkResponse<TcgCard>,
    TcgSetBulkResponse = BulkResponse<TcgSet>,
    VersionBulkResponse = BulkResponse<Version>,
    VersionGroupBulkResponse = BulkResponse<VersionGroup>
)]
pub struct BulkResponse<T: Clone + HasId + Serialize + for<'s> ToSchema<'s>>
{
    pub count: usize,
    pub results: Vec<T>,
}

// Make sure the entities are always sorted by ID
impl<T: Clone + HasId + Serialize + for<'s> ToSchema<'s>> Serialize for BulkResponse<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut sorted_results = self.results.clone();
        sorted_results.sort_by_key(|item| item.id());

        let mut state = serializer.serialize_struct("BulkResponse", 2)?;
        state.serialize_field("count", &self.count)?;
        state.serialize_field("results", &sorted_results)?;
        state.end()
    }
}
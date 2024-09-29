use crate::api::localized_values::{LocalizedValues, VersionGroupedLocalizedValues};
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonMove {
    pub id: i32,
    pub identifier: String,
    /// The generation in which this move was introduced.
    pub generation_id: i32,
    /// The elemental type of this move.
    pub type_id: i32,
    /// The base power of this move. With a value of 0 if it does not have a base power.
    pub power: Option<i32>,
    /// Power points. The number of times this move can be used.
    pub pp: Option<i32>,
    /// The percent value of how likely this move is to be successful.
    pub accuracy: Option<i32>,
    /// A value between -8 and 8. Sets the order in which moves are executed during battle. See Bulbapedia for greater detail.
    pub priority: i32,
    /// The type of target that will receive the effects of the attack.
    pub target_id: i32,
    /// The type of damage the move inflicts on the target, e.g. physical.
    pub damage_class_id: i32,
    pub effect_id: Option<i32>,
    /// The percent value of how likely it is this moves effect will happen.
    pub effect_chance: Option<i32>,
    pub contest_type_id: Option<i32>,
    pub contest_effect_id: Option<i32>,
    pub super_contest_effect_id: Option<i32>,
    pub names: Option<LocalizedValues>,
    pub flavor_texts: Option<VersionGroupedLocalizedValues>,
    pub flag_ids: Vec<i32>,
    /// The category of move this move falls under, e.g. damage or ailment.
    pub category_id: Option<i32>,
    /// The status ailment this move inflicts on its target.
    pub ailment_id: Option<i32>,
    /// The minimum number of times this move hits. Null if it always only hits once.
    pub min_hits: Option<i32>,
    /// The maximum number of times this move hits. Null if it always only hits once.
    pub max_hits: Option<i32>,
    /// The minimum number of turns this move continues to take effect. Null if it always only lasts one turn.
    pub min_turns: Option<i32>,
    /// The maximum number of turns this move continues to take effect. Null if it always only lasts one turn.
    pub max_turns: Option<i32>,
    /// HP drain (if positive) or Recoil damage (if negative), in percent of damage done.
    pub drain: Option<i32>,
    /// The amount of hp gained by the attacking Pokémon, in percent of it's maximum HP.
    pub healing: Option<i32>,
    /// Critical hit rate bonus.
    pub crit_rate: Option<i32>,
    /// The likelihood this attack will cause an ailment.
    pub ailment_chance: Option<i32>,
    /// The likelihood this attack will cause the target Pokémon to flinch.
    pub flinch_chance: Option<i32>,
    /// The likelihood this attack will cause a stat change in the target Pokémon.
    pub stat_chance: Option<i32>,
    /// Amount of change mapped by the id of the affected stat.
    pub stat_changes: Option<HashMap<i32, i32>>,
}

impl HasId for PokemonMove {
    fn id(&self) -> i32 {
        self.id
    }
}
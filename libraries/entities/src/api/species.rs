use crate::api::localized_values::{LocalizedValues, VersionedLocalizedValues};
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Species {
    pub id: i32,
    pub identifier: String,
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub gender_rate: i32,
    /// The base capture rate; up to 255. The higher the number, the easier the catch.
    pub capture_rate: i32,
    /// The happiness when caught by a normal Pokéball; up to 255. The higher the number, the happier the Pokémon.
    pub base_happiness: i32,
    pub is_baby: bool,
    /// Initial hatch counter: one must walk Y × (hatch_counter + 1) steps before this Pokémon's egg hatches, unless utilizing bonuses like Flame Body's. Y varies per generation. In Generations II, III, and VII, Egg cycles are 256 steps long. In Generation IV, Egg cycles are 255 steps long. In Pokémon Brilliant Diamond and Shining Pearl, Egg cycles are also 255 steps long, but are shorter on special dates. In Generations V and VI, Egg cycles are 257 steps long. In Pokémon Sword and Shield, and in Pokémon Scarlet and Violet, Egg cycles are 128 steps long.
    pub hatch_counter: i32,
    pub has_gender_differences: bool,
    pub forms_switchable: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    /// The order in which species should be sorted. Based on National Dex order, except families are grouped together and sorted by stage.
    pub dex_order: i32,
    pub conquest_order: Option<i32>,
    /// The name of this species listed in different languages.
    pub names: Option<LocalizedValues>,
    /// The genus of this Pokémon species listed in multiple languages.
    pub genus: Option<LocalizedValues>,
    /// A list of flavor text entries for this species.
    pub flavor_texts: Option<VersionedLocalizedValues>,
    /// The color of this Pokémon for Pokédex search.
    pub color_id: Option<i32>,
    /// The habitat this species can be encountered in.
    pub habitat_id: Option<i32>,
    /// The rate at which this species gains levels.
    pub growth_rate_id: Option<i32>,
    /// The shape belonging to this species.
    pub shape_id: Option<i32>,
    /// All pokemon which belong to this species.
    pub pokemon_ids: Vec<i32>,
    pub evolution_chain_id: Option<i32>,
    pub evolves_from_species_id: Option<i32>,
}

impl HasId for Species {
    fn id(&self) -> i32 {
        self.id
    }
}
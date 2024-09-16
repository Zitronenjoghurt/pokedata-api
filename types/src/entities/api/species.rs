use crate::entities::api::localized_values::{LocalizedValues, VersionedLocalizedValues};
use crate::entities::api::pokemon::Pokemon;
use crate::entities::csv::pokemon_species::PokemonSpeciesCSV;
use crate::entities::csv::pokemon_species_flavor_text::PokemonSpeciesFlavorTextCSV;
use crate::entities::csv::pokemon_species_names::{PokemonSpeciesGeneraCSV, PokemonSpeciesNamesCSV};
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_id::HasId;
use crate::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use crate::entities::traits::into_versioned_localized_values_map::IntoVersionedLocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Species {
    pub id: u32,
    pub identifier: String,
    /// The chance of this Pokémon being female, in eighths; or -1 for genderless.
    pub gender_rate: i32,
    /// The base capture rate; up to 255. The higher the number, the easier the catch.
    pub capture_rate: u32,
    /// The happiness when caught by a normal Pokéball; up to 255. The higher the number, the happier the Pokémon.
    pub base_happiness: u32,
    pub is_baby: bool,
    /// Initial hatch counter: one must walk Y × (hatch_counter + 1) steps before this Pokémon's egg hatches, unless utilizing bonuses like Flame Body's. Y varies per generation. In Generations II, III, and VII, Egg cycles are 256 steps long. In Generation IV, Egg cycles are 255 steps long. In Pokémon Brilliant Diamond and Shining Pearl, Egg cycles are also 255 steps long, but are shorter on special dates. In Generations V and VI, Egg cycles are 257 steps long. In Pokémon Sword and Shield, and in Pokémon Scarlet and Violet, Egg cycles are 128 steps long.
    pub hatch_counter: u32,
    pub has_gender_differences: bool,
    pub forms_switchable: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    /// The order in which species should be sorted. Based on National Dex order, except families are grouped together and sorted by stage.
    pub dex_order: u32,
    pub conquest_order: Option<u32>,
    /// The name of this species listed in different languages.
    pub names: Option<LocalizedValues>,
    /// The genus of this Pokémon species listed in multiple languages.
    pub genus: Option<LocalizedValues>,
    /// A list of flavor text entries for this species.
    pub flavor_texts: Option<VersionedLocalizedValues>,
    /// The color of this Pokémon for Pokédex search.
    pub color_id: Option<u32>,
    /// The habitat this species can be encountered in.
    pub habitat_id: Option<u32>,
    /// The rate at which this species gains levels.
    pub growth_rate_id: Option<u32>,
    /// The shape belonging to this species.
    pub shape_id: Option<u32>,
    /// All pokemon which belong to this species.
    pub pokemon_ids: Vec<u32>,
    // ToDo: evolution
}

impl HasId for Species {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_species(
    pokemon_species: Vec<PokemonSpeciesCSV>,
    pokemon_map: HashMap<u32, Pokemon>,
    data_path: &PathBuf,
) -> Vec<Species> {
    let names_map = PokemonSpeciesNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let genera_map = PokemonSpeciesGeneraCSV::load(data_path).unwrap().into_localized_values_map();
    let flavor_text_map = PokemonSpeciesFlavorTextCSV::load(data_path).unwrap().into_versioned_localized_values_map();

    let mut species_vec = Vec::with_capacity(pokemon_species.len());

    for entry in pokemon_species {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let species = Species {
            id,
            identifier,
            gender_rate: entry.gender_rate.unwrap_or(0),
            capture_rate: entry.capture_rate.unwrap_or(0),
            base_happiness: entry.base_happiness.unwrap_or(0),
            is_baby: entry.is_baby.unwrap_or(0) == 1,
            hatch_counter: entry.hatch_counter.unwrap_or(0),
            has_gender_differences: entry.has_gender_differences.unwrap_or(0) == 1,
            forms_switchable: entry.forms_switchable.unwrap_or(0) == 1,
            is_legendary: entry.is_legendary.unwrap_or(0) == 1,
            is_mythical: entry.is_mythical.unwrap_or(0) == 1,
            dex_order: entry.order.unwrap_or(0),
            conquest_order: entry.conquest_order,
            names: names_map.get(id),
            genus: genera_map.get(id),
            flavor_texts: flavor_text_map.get(id),
            color_id: entry.color_id,
            growth_rate_id: entry.growth_rate_id,
            habitat_id: entry.habitat_id,
            shape_id: entry.shape_id,
            pokemon_ids: get_pokemon_ids_by_species(&pokemon_map, id),
        };

        species_vec.push(species);
    }

    species_vec
}

fn get_pokemon_ids_by_species(pokemon_map: &HashMap<u32, Pokemon>, species_id: u32) -> Vec<u32> {
    pokemon_map
        .values()
        .filter(|pokemon| pokemon.species_id == species_id)
        .map(|pokemon| pokemon.id)
        .collect()
}
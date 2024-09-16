use crate::entities::api::localized_values::{LocalizedValues, VersionedLocalizedValues};
use crate::entities::csv::pokemon_species::PokemonSpeciesCSV;
use crate::entities::csv::pokemon_species_flavor_text::PokemonSpeciesFlavorTextCSV;
use crate::entities::csv::pokemon_species_names::{PokemonSpeciesGeneraCSV, PokemonSpeciesNamesCSV};
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_id::HasId;
use crate::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use crate::entities::traits::into_versioned_localized_values_map::IntoVersionedLocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Species {
    pub id: u32,
    pub gender_rate: i32,
    pub capture_rate: u32,
    pub base_happiness: u32,
    pub is_baby: bool,
    pub hatch_counter: u32,
    pub has_gender_differences: bool,
    pub forms_switchable: bool,
    pub is_legendary: bool,
    pub is_mythical: bool,
    pub dex_order: u32,
    pub conquest_order: Option<u32>,
    pub names: Option<LocalizedValues>,
    pub genus: Option<LocalizedValues>,
    pub flavor_texts: Option<VersionedLocalizedValues>,
    pub color_id: Option<u32>,
    pub habitat_id: Option<u32>,
    pub growth_rate_id: Option<u32>,
    pub shape_id: Option<u32>,
    // ToDo: evolution
}

impl HasId for Species {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_species(
    pokemon_species: Vec<PokemonSpeciesCSV>,
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

        let species = Species {
            id,
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
        };

        species_vec.push(species);
    }

    species_vec
}
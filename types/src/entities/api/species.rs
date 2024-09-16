use crate::entities::api::pokemon_color::PokemonColor;
use crate::entities::api::pokemon_shape::PokemonShape;
use crate::entities::csv::pokemon_species::PokemonSpeciesCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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
    pub color: Option<PokemonColor>,
    pub shape: Option<PokemonShape>,
    // ToDo: evolution
}

impl HasId for Species {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_species(
    pokemon_species: Vec<PokemonSpeciesCSV>,
    colors: HashMap<u32, PokemonColor>,
    shapes: HashMap<u32, PokemonShape>,
) -> Vec<Species> {
    let mut species_vec = Vec::with_capacity(pokemon_species.len());

    for entry in pokemon_species {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let color = match entry.color_id {
            Some(color_id) => colors.get(&color_id).cloned(),
            None => None,
        };

        let shape = match entry.shape_id {
            Some(shape_id) => shapes.get(&shape_id).cloned(),
            None => None,
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
            color,
            shape,
        };

        species_vec.push(species);
    }

    species_vec
}
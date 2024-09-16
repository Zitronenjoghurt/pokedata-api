use crate::entities::csv::pokemon::PokemonCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Pokemon {
    pub id: u32,
    pub identifier: String,
    /// The species this Pokémon belongs to.
    pub species_id: u32,
    pub height_decimeter: u32,
    pub weight_hectograms: u32,
    /// The base experience gained for defeating this Pokémon.
    pub base_experience: u32,
    /// Order for sorting. Almost national order, except families are grouped together.
    pub dex_order: u32,
    /// Set for exactly one Pokémon used as the default for each species.
    pub is_default: bool,
}

impl HasId for Pokemon {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_pokemon(pokemon_csv: Vec<PokemonCSV>) -> Vec<Pokemon> {
    let mut pokemon_vec = Vec::with_capacity(pokemon_csv.len());

    for entry in pokemon_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let species_id = match entry.species_id {
            Some(species_id) => species_id,
            None => continue,
        };

        let pokemon = Pokemon {
            id,
            identifier,
            species_id,
            height_decimeter: entry.height.unwrap_or(0),
            weight_hectograms: entry.weight.unwrap_or(0),
            base_experience: entry.base_experience.unwrap_or(0),
            dex_order: entry.order.unwrap_or(0),
            is_default: entry.is_default.unwrap_or(0) == 1,
        };

        pokemon_vec.push(pokemon);
    }

    pokemon_vec
}
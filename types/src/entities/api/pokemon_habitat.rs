use crate::entities::api::localized_values::{LocalizedValues, LocalizedValuesMap};
use crate::entities::csv::pokemon_habitats::PokemonHabitatsCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct PokemonHabitat {
    pub id: u32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for PokemonHabitat {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_pokemon_habitats(
    pokemon_habitats_csv: Vec<PokemonHabitatsCSV>,
    names_map: LocalizedValuesMap,
) -> Vec<PokemonHabitat> {
    let mut habitats = Vec::with_capacity(pokemon_habitats_csv.len());

    for entry in pokemon_habitats_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let habitat = PokemonHabitat {
            id,
            identifier,
            names: names_map.get(id),
        };

        habitats.push(habitat);
    }

    habitats
}
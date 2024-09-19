use crate::entities::api::localized_values::{LocalizedValues, LocalizedValuesMap};
use crate::entities::csv::pokemon_colors::PokemonColorsCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonColor {
    pub id: u32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for PokemonColor {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_pokemon_colors(
    pokemon_colors_csv: Vec<PokemonColorsCSV>,
    names_map: LocalizedValuesMap,
) -> Vec<PokemonColor> {
    let mut colors = Vec::with_capacity(pokemon_colors_csv.len());

    for entry in pokemon_colors_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let color = PokemonColor {
            id,
            identifier,
            names: names_map.get(id),
        };

        colors.push(color);
    }

    colors
}
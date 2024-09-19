use crate::entities::api::localized_values::LocalizedValues;
use crate::entities::csv::pokemon_shape_prose::{PokemonShapeAwesomeNamesCSV, PokemonShapeDescriptionsCSV, PokemonShapeNamesCSV};
use crate::entities::csv::pokemon_shapes::PokemonShapesCSV;
use crate::entities::csv_entity::CSVEntity;
use crate::entities::traits::has_id::HasId;
use crate::entities::traits::into_localized_values_map::IntoLocalizedValuesMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct PokemonShape {
    pub id: u32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
    /// The "scientific" name of this Pokémon shape listed in different languages.
    pub awesome_names: Option<LocalizedValues>,
    pub descriptions: Option<LocalizedValues>,
}

impl HasId for PokemonShape {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_pokemon_shapes(
    pokemon_shapes_csv: Vec<PokemonShapesCSV>,
    data_path: &PathBuf,
) -> Vec<PokemonShape> {
    let names_map = PokemonShapeNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let awesome_names_map = PokemonShapeAwesomeNamesCSV::load(data_path).unwrap().into_localized_values_map();
    let descriptions_map = PokemonShapeDescriptionsCSV::load(data_path).unwrap().into_localized_values_map();

    let mut shapes = Vec::with_capacity(pokemon_shapes_csv.len());

    for entry in pokemon_shapes_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let color = PokemonShape {
            id,
            identifier,
            names: names_map.get(id),
            awesome_names: awesome_names_map.get(id),
            descriptions: descriptions_map.get(id),
        };

        shapes.push(color);
    }

    shapes
}
use crate::entities::api::localized_values::{LocalizedValues, LocalizedValuesMap};
use crate::entities::csv::generations::GenerationsCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct Generation {
    pub id: u32,
    pub main_region_id: u32,
    pub identifier: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for Generation {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_generations(
    generations_csv: Vec<GenerationsCSV>,
    names_map: LocalizedValuesMap,
) -> Vec<Generation> {
    let mut generations = Vec::with_capacity(generations_csv.len());

    for entry in generations_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let main_region_id = match entry.main_region_id {
            Some(main_region_id) => main_region_id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let generation = Generation {
            id,
            main_region_id,
            identifier,
            names: names_map.get(id),
        };

        generations.push(generation);
    }

    generations
}
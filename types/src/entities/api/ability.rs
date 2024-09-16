use crate::entities::csv::abilities::AbilitiesCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ability {
    pub id: u32,
    pub is_main_series: bool,
}

impl HasId for Ability {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_abilities(abilities_csv: Vec<AbilitiesCSV>) -> Vec<Ability> {
    let mut abilities_vec = Vec::with_capacity(abilities_csv.len());

    for entry in abilities_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let ability = Ability {
            id,
            is_main_series: entry.is_main_series.unwrap_or(0) == 1,
        };

        abilities_vec.push(ability);
    }

    abilities_vec
}
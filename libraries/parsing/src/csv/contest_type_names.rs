use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use crate::traits::into_localized_values_map::IntoLocalizedValuesMap;
use pokedata_api_entities::api::berry_flavor::BerryFlavor;
use pokedata_api_utils::constants::ENGLISH_LANGUAGE_ID;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BerryFlavorNamesCSV {
    pub contest_type_id: i32,
    pub local_language_id: i32,
    pub flavor: String,
}

impl CSVEntity for BerryFlavorNamesCSV {
    fn file_name() -> &'static str {
        "contest_type_names"
    }
}

impl HasLocalizedValues for BerryFlavorNamesCSV {
    fn id(&self) -> i32 {
        self.contest_type_id
    }

    fn language_id(&self) -> i32 {
        self.local_language_id
    }

    fn name(&self) -> String {
        self.flavor.clone()
    }
}

/// A bit cheesy, but hey, it's not my data im working with here
/// The berry flavors are only stored in the contest_type_names table as they're directly connected to a contest type
pub fn convert_to_berry_flavors(berry_flavor_names_csv_entries: Vec<BerryFlavorNamesCSV>) -> Vec<BerryFlavor> {
    berry_flavor_names_csv_entries
        .into_localized_values_map()
        .0
        .into_iter()
        .map(|(id, names)| BerryFlavor {
            id,
            identifier: names.get(ENGLISH_LANGUAGE_ID).unwrap_or_default().to_lowercase(),
            names: Some(names),
            contest_type_id: id,
        })
        .collect()
}
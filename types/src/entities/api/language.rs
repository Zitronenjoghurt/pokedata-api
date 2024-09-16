use crate::entities::api::localized_values::{LocalizedValues, LocalizedValuesMap};
use crate::entities::csv::languages::LanguagesCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct Language {
    pub id: u32,
    pub iso639: String,
    pub iso3166: String,
    pub official: bool,
    pub order: u32,
    pub names: Option<LocalizedValues>,
}

impl HasId for Language {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_languages(
    languages_csv: Vec<LanguagesCSV>,
    names_map: LocalizedValuesMap,
) -> Vec<Language> {
    let mut languages_vec = Vec::with_capacity(languages_csv.len());

    for entry in languages_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let iso639 = match entry.iso639 {
            Some(iso639) => iso639,
            None => continue,
        };

        let iso3166 = match entry.iso3166 {
            Some(iso3166) => iso3166,
            None => continue,
        };

        let official = match entry.official {
            Some(official) => official == 1,
            None => continue,
        };

        let order = match entry.order {
            Some(order) => order,
            None => continue,
        };

        let language = Language {
            id,
            iso639,
            iso3166,
            official,
            order,
            names: names_map.get(id),
        };

        languages_vec.push(language);
    }

    languages_vec
}
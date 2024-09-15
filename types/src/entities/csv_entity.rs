use crate::entities::csv::abilities::Abilities;
use std::collections::HashMap;

pub fn get_all_entities() -> Vec<Box<dyn CSVEntity>> {
    vec![
        Box::new(Abilities::default())
    ]
}

pub trait CSVEntity {
    fn get_file_name(&self) -> &'static str;

    fn get_base_download_url(&self) -> &'static str {
        "https://github.com/PokeAPI/pokeapi/blob/master/data/v2/csv/"
    }

    fn is_downloadable(&self) -> bool {
        true
    }

    fn get_download_url(&self) -> String {
        format!("{}{}.csv", self.get_base_download_url(), self.get_file_name())
    }
}

pub fn get_download_map() -> HashMap<&'static str, String> {
    get_all_entities()
        .into_iter()
        .filter(|entity| entity.is_downloadable())
        .map(|entity| (entity.get_file_name(), entity.get_download_url()))
        .collect()
}
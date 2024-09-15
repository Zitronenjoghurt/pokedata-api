use pokedata_api_types::entities::api::ability::Ability;
use pokedata_api_types::entities::csv::abilities::AbilitiesCSV;
use pokedata_api_types::entities::csv_entity::CSVEntity;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn build(data_path: &PathBuf) -> HashMap<u32, Ability> {
    let abilities = AbilitiesCSV::load(data_path).unwrap();
    let mut map: HashMap<u32, Ability> = HashMap::new();

    for abilities_entry in abilities {
        let ability = Ability {
            id: abilities_entry.id,
            generation_id: abilities_entry.generation_id,
            is_main_series: abilities_entry.is_main_series == 1,
        };
        map.insert(abilities_entry.id, ability);
    }

    map
}
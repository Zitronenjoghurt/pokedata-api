use crate::json::tcg_card::TcgCardJSON;
use crate::json::tcg_set::TcgSetJSON;
use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::api::tcg_set::TcgSet;
use pokedata_api_entities::traits::into_id_map::IntoIdMap;
use pokedata_api_utils::directories::{tcg_repository_cards_path, tcg_repository_sets_path};
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::io::Read;
use std::path::PathBuf;

fn load_json_from_directory<T: DeserializeOwned>(path: &PathBuf) -> Result<Vec<T>, Box<dyn Error>> {
    let mut all_items = Vec::new();

    for entry in fs::read_dir(path)? {
        let file_path = entry?.path();
        if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("json") {
            let mut contents = String::new();
            fs::File::open(file_path)?.read_to_string(&mut contents)?;
            all_items.extend(serde_json::from_str::<Vec<T>>(&contents)?);
        }
    }

    Ok(all_items)
}

pub fn load_tcg_cards() -> HashMap<i32, TcgCard> {
    load_cards_json().unwrap().into_iter().enumerate()
        .map(|(index, card_json)| card_json.convert_to_card(index as i32))
        .collect::<Vec<TcgCard>>().into_id_map()
}

pub fn load_tcg_sets() -> HashMap<i32, TcgSet> {
    load_sets_json().unwrap().into_iter().enumerate()
        .map(|(index, set_json)| set_json.convert_to_set(index as i32))
        .collect::<Vec<TcgSet>>().into_id_map()
}

fn load_cards_json() -> Result<Vec<TcgCardJSON>, Box<dyn Error>> {
    load_json_from_directory(&tcg_repository_cards_path())
}

fn load_sets_json() -> Result<Vec<TcgSetJSON>, Box<dyn Error>> {
    load_json_from_directory(&tcg_repository_sets_path())
}
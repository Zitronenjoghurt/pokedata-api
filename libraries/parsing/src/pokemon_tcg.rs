use crate::json::tcg_card::TcgCardJSON;
use pokedata_api_entities::api::tcg_card::TcgCard;
use pokedata_api_entities::traits::into_id_map::IntoIdMap;
use pokedata_api_utils::directories::tcg_repository_cards_path;
use std::collections::HashMap;
use std::fs;
use std::io::Read;

pub fn load_tcg_data() -> HashMap<i32, TcgCard> {
    let cards: Vec<TcgCard> = load_cards()
        .unwrap()
        .into_iter()
        .enumerate()
        .map(|(index, card_json)| card_json.convert_to_card(index as i32))
        .collect();

    cards.into_id_map()
}

pub fn load_cards() -> Result<Vec<TcgCardJSON>, Box<dyn std::error::Error>> {
    let path = tcg_repository_cards_path();
    let mut all_cards = Vec::new();

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_path = entry.path();

        if file_path.is_file() && file_path.extension().and_then(|s| s.to_str()) == Some("json") {
            let mut file = fs::File::open(file_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;

            let cards: Vec<TcgCardJSON> = serde_json::from_str(&contents)?;
            all_cards.extend(cards);
        }
    }

    Ok(all_cards)
}
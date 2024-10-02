use crate::json::tcg_legalities::TcgLegalitiesJSON;
use crate::json::tcg_set_images::TcgSetImagesJSON;
use pokedata_api_entities::api::tcg_set::TcgSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgSetJSON {
    pub id: String,
    pub name: String,
    pub series: String,
    #[serde(rename = "printedTotal")]
    pub printed_total: i32,
    pub total: i32,
    pub legalities: TcgLegalitiesJSON,
    #[serde(rename = "ptcgoCode")]
    pub ptcgo_code: Option<String>,
    #[serde(rename = "releaseDate")]
    pub release_date: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
    pub images: TcgSetImagesJSON,
}

impl TcgSetJSON {
    pub fn convert_to_set(self, index: i32) -> TcgSet {
        TcgSet {
            id: index,
            identifier: self.id,
            name: self.name,
            printed_total: self.printed_total,
            total: self.total,
            legalities: self.legalities.into(),
            ptcgo_code: self.ptcgo_code,
            release_date: self.release_date,
            updated_at: self.updated_at,
            images: self.images.into(),
            card_ids: vec![],
        }
    }
}
use pokedata_api_entities::api::tcg_set::TcgSetImages;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgSetImagesJSON {
    pub symbol: String,
    pub logo: String,
}

impl Into<TcgSetImages> for TcgSetImagesJSON {
    fn into(self) -> TcgSetImages {
        TcgSetImages {
            symbol: self.symbol,
            logo: self.logo,
        }
    }
}
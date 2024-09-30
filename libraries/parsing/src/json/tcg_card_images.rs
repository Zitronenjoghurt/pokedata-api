use pokedata_api_entities::api::tcg_card::TcgCardImages;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct TcgCardImagesJSON {
    pub small: String,
    pub large: String,
}

impl Into<TcgCardImages> for TcgCardImagesJSON {
    fn into(self) -> TcgCardImages {
        TcgCardImages {
            small: self.small,
            large: self.large,
        }
    }
}
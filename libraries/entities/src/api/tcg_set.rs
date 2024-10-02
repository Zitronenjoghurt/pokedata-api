use crate::api::tcg_legalities::TcgLegalities;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgSet {
    pub id: i32,
    pub identifier: String,
    pub name: String,
    pub printed_total: i32,
    pub total: i32,
    pub legalities: TcgLegalities,
    pub ptcgo_code: Option<String>,
    pub release_date: String,
    pub updated_at: String,
    pub images: TcgSetImages,
    pub card_ids: Vec<i32>,
}

impl HasId for TcgSet {
    fn id(&self) -> i32 {
        self.id.clone()
    }
}

#[derive(Clone, Debug, Default, PartialOrd, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct TcgSetImages {
    pub symbol: String,
    pub logo: String,
}
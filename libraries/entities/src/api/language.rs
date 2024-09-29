use crate::api::localized_values::LocalizedValues;
use crate::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
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
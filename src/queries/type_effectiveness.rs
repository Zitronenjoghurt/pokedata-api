use crate::serde::comma_seperated::deserialize_comma_separated;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct AllTypeEffectivenessQuery {
    /// Comma seperated
    #[serde(default, deserialize_with = "deserialize_comma_separated")]
    pub defending_ids: Vec<u32>,
    pub generation_id: Option<u32>,
}

#[derive(Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct TypeEffectivenessQuery {
    /// Comma seperated
    pub attacking_id: u32,
    /// Comma seperated
    #[serde(default, deserialize_with = "deserialize_comma_separated")]
    pub defending_ids: Vec<u32>,
    pub generation_id: Option<u32>,
}
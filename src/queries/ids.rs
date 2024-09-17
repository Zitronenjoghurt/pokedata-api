use crate::serde::comma_seperated::deserialize_optional_comma_separated;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct IdsQuery {
    #[serde(default, deserialize_with = "deserialize_optional_comma_separated")]
    pub ids: Option<Vec<u32>>,
}
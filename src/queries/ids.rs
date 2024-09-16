use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct IdsQuery {
    pub ids: Option<Vec<u32>>,
}
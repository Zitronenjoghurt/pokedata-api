use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Default, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct IdsQuery {
    #[serde(default, deserialize_with = "deserialize_optional_comma_separated")]
    pub ids: Option<Vec<u32>>,
}

fn deserialize_optional_comma_separated<'de, D>(deserializer: D) -> Result<Option<Vec<u32>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        None => Ok(None),
        Some(s) if s.is_empty() => Ok(Some(Vec::new())),
        Some(s) => s
            .split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map(Some)
            .map_err(serde::de::Error::custom),
    }
}
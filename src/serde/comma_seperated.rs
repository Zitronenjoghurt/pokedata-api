use serde::Deserialize;

pub fn deserialize_comma_separated<'de, D>(deserializer: D) -> Result<Vec<u32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(Vec::new())
    } else {
        s.split(',')
            .map(str::parse)
            .collect::<Result<Vec<_>, _>>()
            .map_err(serde::de::Error::custom)
    }
}

pub fn deserialize_optional_comma_separated<'de, D>(deserializer: D) -> Result<Option<Vec<u32>>, D::Error>
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
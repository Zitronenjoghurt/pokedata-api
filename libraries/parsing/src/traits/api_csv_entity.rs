use crate::csv_entity::CSVEntity;
use std::error::Error;
use std::path::PathBuf;

/// Implemented by CSV Entities that can be converted to API entities
pub trait ApiCSVEntity: CSVEntity {
    type ApiType;
    type ConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>>;

    fn load_and_convert(
        base_path: &PathBuf,
        conversion_data: &Self::ConversionData,
    ) -> Result<Vec<Self::ApiType>, Box<dyn Error>>
    where
        Self: Sized,
    {
        let csv_entities = Self::load(base_path)?;
        csv_entities
            .into_iter()
            .map(|entity| Self::convert(entity, conversion_data))
            .collect()
    }
}
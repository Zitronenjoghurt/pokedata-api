use crate::entities::api::localized_values::{LocalizedValues, LocalizedValuesMap};
use crate::entities::csv::growth_rates::GrowthRatesCSV;
use crate::entities::traits::has_id::HasId;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Serialize, Deserialize, ToSchema)]
pub struct GrowthRate {
    pub id: u32,
    pub identifier: String,
    pub formula: String,
    pub names: Option<LocalizedValues>,
}

impl HasId for GrowthRate {
    fn id(&self) -> u32 {
        self.id
    }
}

pub fn build_growth_rates(
    growth_rates_csv: Vec<GrowthRatesCSV>,
    names_map: LocalizedValuesMap,
) -> Vec<GrowthRate> {
    let mut growth_rates = Vec::with_capacity(growth_rates_csv.len());

    for entry in growth_rates_csv {
        let id = match entry.id {
            Some(id) => id,
            None => continue,
        };

        let identifier = match entry.identifier {
            Some(identifier) => identifier,
            None => continue,
        };

        let formula = match entry.formula {
            Some(formula) => formula,
            None => continue,
        };

        let growth_rate = GrowthRate {
            id,
            identifier,
            formula,
            names: names_map.get(id),
        };

        growth_rates.push(growth_rate);
    }

    growth_rates
}
use crate::csv_entity::CSVEntity;
use crate::traits::has_localized_values::HasLocalizedValues;
use crate::traits::has_version_group_id::HasVersionGroupId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MoveFlavorTextCSV {
    pub move_id: u32,
    pub version_group_id: u32,
    pub language_id: u32,
    pub flavor_text: String,
}

impl CSVEntity for MoveFlavorTextCSV {
    fn file_name() -> &'static str {
        "move_flavor_text"
    }
}

impl HasVersionGroupId for MoveFlavorTextCSV {
    fn version_group_id(&self) -> u32 {
        self.version_group_id
    }
}

impl HasLocalizedValues for MoveFlavorTextCSV {
    fn id(&self) -> u32 {
        self.move_id
    }

    fn language_id(&self) -> u32 {
        self.language_id
    }

    fn name(&self) -> String {
        self.flavor_text.clone()
    }
}
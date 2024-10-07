use crate::csv_entity::CSVEntity;
use pokedata_api_entities::api::machine::Machine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct MachinesCSV {
    pub machine_number: i32,
    pub version_group_id: i32,
    pub item_id: i32,
    pub move_id: i32,
}

impl CSVEntity for MachinesCSV {
    fn file_name() -> &'static str {
        "machines"
    }
}

impl MachinesCSV {
    pub fn convert_to_machine(&self, index: i32) -> Machine {
        Machine {
            id: index,
            machine_number: self.machine_number,
            version_group_id: self.version_group_id,
            item_id: self.item_id,
            move_id: self.move_id,
        }
    }
}

pub fn convert_to_machines(mut machines_csv_entries: Vec<MachinesCSV>) -> Vec<Machine> {
    machines_csv_entries.sort_by_key(|entry| entry.machine_number);

    machines_csv_entries.iter().enumerate().map(|(index, entry)| {
        entry.convert_to_machine(index as i32)
    }).collect()
}
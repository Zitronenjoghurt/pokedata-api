use crate::csv_entity::CSVEntity;
use crate::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_entities::api::evolution::Evolution;
use pokedata_api_entities::api::evolution_chain::{EvolutionChain, EvolutionChainNode};
use pokedata_api_entities::api::species::Species;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::error::Error;

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct EvolutionChainsCSV {
    pub id: u32,
    pub baby_trigger_item_id: Option<u32>,
}

impl CSVEntity for EvolutionChainsCSV {
    fn file_name() -> &'static str {
        "evolution_chains"
    }
}

impl ApiCSVEntity for EvolutionChainsCSV {
    type ApiType = EvolutionChain;
    type ConversionData = EvolutionChainConversionData;

    fn convert(entry: Self, data: &Self::ConversionData) -> Result<Self::ApiType, Box<dyn Error>> {
        let species: HashMap<u32, &Species> = data.species_map
            .iter()
            .filter(|(_, species)| species.evolution_chain_id == Some(entry.id))
            .map(|(&id, species)| (id, species))
            .collect();

        let evolutions: HashMap<u32, &Evolution> = data.evolutions_map
            .iter()
            .filter(|(_, evolution)| species.contains_key(&evolution.evolved_species_id))
            .map(|(&id, evolution)| (id, evolution))
            .collect();

        let evolved_species_ids: HashSet<u32> = evolutions.values()
            .map(|e| e.evolved_species_id)
            .collect();

        let root_species_id = species.keys()
            .find(|&&id| !evolved_species_ids.contains(&id))
            .ok_or("No root species found")?;

        let root_node = create_evolution_chain_node(*root_species_id, &species, &evolutions)?;

        Ok(EvolutionChain {
            id: entry.id,
            baby_trigger_item_id: entry.baby_trigger_item_id,
            chain: root_node,
        })
    }
}

fn create_evolution_chain_node(
    species_id: u32,
    species_map: &HashMap<u32, &Species>,
    evolutions_map: &HashMap<u32, &Evolution>,
) -> Result<EvolutionChainNode, Box<dyn Error>> {
    let evolution_to_this_species = evolutions_map.values()
        .find(|&e| e.evolved_species_id == species_id);

    let mut node = EvolutionChainNode {
        species_id,
        evolution_id: evolution_to_this_species.map(|e| e.id),
        next: vec![],
    };

    let next_species: Vec<u32> = evolutions_map.values()
        .filter(|&e| species_map.get(&e.evolved_species_id)
            .and_then(|s| s.evolves_from_species_id) == Some(species_id))
        .map(|e| e.evolved_species_id)
        .collect();

    for next_id in next_species {
        let next_node = create_evolution_chain_node(next_id, species_map, evolutions_map)?;
        node.next.push(next_node);
    }

    Ok(node)
}

#[derive(Debug, Default)]
pub struct EvolutionChainConversionData {
    pub evolutions_map: HashMap<u32, Evolution>,
    pub species_map: HashMap<u32, Species>,
}

impl EvolutionChainConversionData {
    pub fn load(evolutions: &HashMap<u32, Evolution>, species: &HashMap<u32, Species>) -> Self {
        Self {
            evolutions_map: evolutions.clone(),
            species_map: species.clone(),
        }
    }
}
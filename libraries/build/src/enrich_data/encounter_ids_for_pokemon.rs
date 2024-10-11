use pokedata_api_entities::api::encounter::Encounter;
use pokedata_api_entities::api::pokemon::Pokemon;
use std::collections::HashMap;

pub fn enrich_pokemon_with_encounter_ids(
    pokemon_map: &mut HashMap<i32, Pokemon>,
    encounters: &HashMap<i32, Encounter>,
)
{
    for encounter in encounters.values() {
        let Some(pokemon) = pokemon_map.get_mut(&encounter.location_area_id) else { continue };
        pokemon.encounter_ids.push(encounter.id);
    }
}
use pokedata_api_entities::api::encounter::Encounter;
use pokedata_api_entities::api::location_area::LocationArea;
use std::collections::HashMap;

pub fn enrich_locations_areas_with_encounter_ids(
    encounters: &HashMap<i32, Encounter>,
    location_areas: &mut HashMap<i32, LocationArea>,
)
{
    for encounter in encounters.values() {
        let Some(area) = location_areas.get_mut(&encounter.location_area_id) else { continue };
        area.encounter_ids.push(area.id);
    }
}
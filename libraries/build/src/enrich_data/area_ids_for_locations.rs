use pokedata_api_entities::api::location::Location;
use pokedata_api_entities::api::location_area::LocationArea;
use std::collections::HashMap;

pub fn enrich_locations_with_area_ids(
    locations: &mut HashMap<i32, Location>,
    location_areas: &HashMap<i32, LocationArea>,
)
{
    for area in location_areas.values() {
        let Some(location) = locations.get_mut(&area.location_id) else { continue };
        location.location_area_ids.push(area.id);
    }
}
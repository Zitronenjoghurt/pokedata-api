use crate::get_app_state;
use crate::models::bulk_response::LocationAreaBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_location_areas() {
    let state = get_app_state();
    let response: LocationAreaBulkResponse = test_get("/location-area").await.unwrap();

    let size = state.location_areas.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_location_area() {
    let response: LocationAreaBulkResponse = test_get("/location-area?ids=6").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 6);
    assert_eq!(test_entity.location_id, 6);
    assert_eq!(test_entity.game_index, 6);
    assert_eq!(test_entity.identifier, Some("1f".to_string()));
    assert_eq!(names.get(9), Some("Oreburgh mine (1F)".to_string()));

    assert_eq!(test_entity.encounter_rates.len(), 3);
    assert_eq!(test_entity.encounter_rates.get(&12), Some(&(1, 10)));
    assert_eq!(test_entity.encounter_rates.get(&13), Some(&(1, 10)));
    assert_eq!(test_entity.encounter_rates.get(&14), Some(&(1, 10)));
}
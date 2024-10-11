use crate::get_app_state;
use crate::models::bulk_response::LocationBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_locations() {
    let state = get_app_state();
    let response: LocationBulkResponse = test_get("/location").await.unwrap();

    let size = state.locations.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_location() {
    let response1: LocationBulkResponse = test_get("/location?ids=747").await.unwrap();
    let test_entity1 = response1.results.first().cloned().unwrap_or_default();

    let names = test_entity1.names.unwrap_or_default();
    let subtitles = test_entity1.subtitles.unwrap_or_default();
    assert_eq!(response1.count, 1);
    assert_eq!(test_entity1.id, 747);
    assert_eq!(test_entity1.region_id, Some(7));
    assert_eq!(test_entity1.identifier, "wela-volcano-park--totems-den");
    assert_eq!(names.get(9), Some("Wela Volcano Park".to_string()));
    assert_eq!(subtitles.get(9), Some("Totem’s Den".to_string()));
    assert!(test_entity1.game_indices.contains(&84));
    assert_eq!(test_entity1.game_indices.len(), 1);
    assert!(test_entity1.generation_ids.contains(&7));
    assert_eq!(test_entity1.generation_ids.len(), 1);

    let response2: LocationBulkResponse = test_get("/location?ids=6").await.unwrap();
    let test_entity2 = response2.results.first().cloned().unwrap_or_default();

    assert!(test_entity2.location_area_ids.contains(&6));
    assert!(test_entity2.location_area_ids.contains(&7));
    assert_eq!(test_entity2.location_area_ids.len(), 2);
}
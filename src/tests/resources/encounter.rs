use crate::get_app_state;
use crate::models::bulk_response::EncounterBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_encounters() {
    let state = get_app_state();
    let response: EncounterBulkResponse = test_get("/encounter").await.unwrap();

    let size = state.encounters.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_encounter() {
    let response: EncounterBulkResponse = test_get("/encounter?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.version_id, 12);
    assert_eq!(test_entity.location_area_id, 1);
    assert_eq!(test_entity.encounter_slot_id, 28);
    assert_eq!(test_entity.pokemon_id, 72);
    assert_eq!(test_entity.min_level, 20);
    assert_eq!(test_entity.max_level, 30);
}
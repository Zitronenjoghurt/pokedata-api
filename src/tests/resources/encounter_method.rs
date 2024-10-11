use crate::get_app_state;
use crate::models::bulk_response::EncounterMethodBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_encounter_methods() {
    let state = get_app_state();
    let response: EncounterMethodBulkResponse = test_get("/encounter-method").await.unwrap();

    let size = state.encounter_methods.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_encounter_method() {
    let response: EncounterMethodBulkResponse = test_get("/encounter-method?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "walk");
    assert_eq!(test_entity.order, 1);
    assert_eq!(names.get(9), Some("Walking in tall grass or a cave".to_string()));
}
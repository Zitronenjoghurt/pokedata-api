use crate::get_app_state;
use crate::models::bulk_response::EncounterConditionValueBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_encounter_condition_values() {
    let state = get_app_state();
    let response: EncounterConditionValueBulkResponse = test_get("/encounter-condition-value").await.unwrap();

    let size = state.encounter_condition_values.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_encounter_condition_value() {
    let response: EncounterConditionValueBulkResponse = test_get("/encounter-condition-value?ids=2").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 2);
    assert_eq!(test_entity.encounter_condition_id, 1);
    assert!(test_entity.is_default);
    assert_eq!(test_entity.identifier, "swarm-no");
    assert_eq!(names.get(9), Some("Not during a swarm".to_string()));
}
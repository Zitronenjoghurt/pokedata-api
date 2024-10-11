use crate::get_app_state;
use crate::models::bulk_response::EncounterConditionBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_encounter_conditions() {
    let state = get_app_state();
    let response: EncounterConditionBulkResponse = test_get("/encounter-condition").await.unwrap();

    let size = state.encounter_conditions.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_encounter_condition() {
    let response: EncounterConditionBulkResponse = test_get("/encounter-condition?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "swarm");
    assert_eq!(names.get(9), Some("Swarm".to_string()));
    assert!(test_entity.condition_value_ids.contains(&1));
    assert!(test_entity.condition_value_ids.contains(&2));
    assert_eq!(test_entity.condition_value_ids.len(), 2);
}
use crate::get_app_state;
use crate::models::bulk_response::EvolutionTriggerBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_evolution_triggers() {
    let state = get_app_state();
    let response: EvolutionTriggerBulkResponse = test_get("/evolution-trigger").await.unwrap();

    let size = state.evolution_triggers.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_evolution_trigger() {
    let response: EvolutionTriggerBulkResponse = test_get("/evolution-trigger?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "level-up".to_string());
    assert_eq!(names.get(9), Some("Level up".to_string()))
}
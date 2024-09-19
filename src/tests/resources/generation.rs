use crate::get_app_state;
use crate::models::bulk_response::GenerationBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_generations() {
    let state = get_app_state();
    let response: GenerationBulkResponse = test_get("/generation").await.unwrap();

    let size = state.generations.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_generation() {
    let response: GenerationBulkResponse = test_get("/generation?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "generation-i");
    assert_eq!(test_entity.main_region_id, 1);
    assert_eq!(names.get(9), Some("Generation I".to_string()))
}
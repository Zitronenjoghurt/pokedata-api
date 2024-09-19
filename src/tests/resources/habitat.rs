use crate::get_app_state;
use crate::models::bulk_response::HabitatBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_habitats() {
    let state = get_app_state();
    let response: HabitatBulkResponse = test_get("/habitat").await.unwrap();

    let size = state.habitats.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_habitat() {
    let response: HabitatBulkResponse = test_get("/habitat?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "cave");
    assert_eq!(names.get(9), Some("cave".to_string()))
}
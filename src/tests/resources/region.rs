use crate::get_app_state;
use crate::models::bulk_response::RegionBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_regions() {
    let state = get_app_state();
    let response: RegionBulkResponse = test_get("/region").await.unwrap();

    let size = state.regions.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_stat() {
    let response: RegionBulkResponse = test_get("/region?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "kanto");
    assert_eq!(names.get(9), Some("Kanto".to_string()));
}
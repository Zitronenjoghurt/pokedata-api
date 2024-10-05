use crate::get_app_state;
use crate::models::bulk_response::BerryFlavorBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_berry_flavors() {
    let state = get_app_state();
    let response: BerryFlavorBulkResponse = test_get("/berry-flavor").await.unwrap();

    let size = state.berry_flavors.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_berry_flavor() {
    let response: BerryFlavorBulkResponse = test_get("/berry-flavor?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.contest_type_id, 1);
    assert_eq!(test_entity.identifier, "spicy");
    assert_eq!(names.get(9), Some("Spicy".to_string()));
}
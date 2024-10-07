use crate::get_app_state;
use crate::models::bulk_response::ItemFlagBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_item_flags() {
    let state = get_app_state();
    let response: ItemFlagBulkResponse = test_get("/item-flag").await.unwrap();

    let size = state.item_flags.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_item_flag() {
    let response: ItemFlagBulkResponse = test_get("/item-flag?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    let descriptions = test_entity.descriptions.unwrap_or_default();
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "countable");
    assert_eq!(names.get(9), Some("Countable".to_string()));
    assert_eq!(descriptions.get(9), Some("Has a count in the bag".to_string()));
}
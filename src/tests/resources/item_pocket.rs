use crate::get_app_state;
use crate::models::bulk_response::ItemPocketBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_item_pockets() {
    let state = get_app_state();
    let response: ItemPocketBulkResponse = test_get("/item-pocket").await.unwrap();

    let size = state.item_pockets.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_item_pocket() {
    let response: ItemPocketBulkResponse = test_get("/item-pocket?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "misc");
    assert_eq!(names.get(9), Some("Items".to_string()));
}
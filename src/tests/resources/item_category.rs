use crate::get_app_state;
use crate::models::bulk_response::ItemCategoryBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_item_categories() {
    let state = get_app_state();
    let response: ItemCategoryBulkResponse = test_get("/item-category").await.unwrap();

    let size = state.item_categories.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_item_category() {
    let response: ItemCategoryBulkResponse = test_get("/item-category?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.pocket_id, 7);
    assert_eq!(test_entity.identifier, "stat-boosts");
    assert_eq!(names.get(9), Some("Stat boosts".to_string()));
}
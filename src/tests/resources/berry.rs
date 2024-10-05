use crate::get_app_state;
use crate::models::bulk_response::BerryBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_berries() {
    let state = get_app_state();
    let response: BerryBulkResponse = test_get("/berry").await.unwrap();

    let size = state.berries.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_berry() {
    let response: BerryBulkResponse = test_get("/berry?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.item_id, 126);
    assert_eq!(test_entity.firmness_id, 2);
    assert_eq!(test_entity.natural_gift_power, 60);
    assert_eq!(test_entity.natural_gift_type_id, 10);
    assert_eq!(test_entity.size_millimeter, 20);
    assert_eq!(test_entity.max_harvest, 5);
    assert_eq!(test_entity.growth_time, 3);
    assert_eq!(test_entity.soil_dryness, 15);
    assert_eq!(test_entity.smoothness, 25);
}
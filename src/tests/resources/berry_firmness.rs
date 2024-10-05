use crate::get_app_state;
use crate::models::bulk_response::BerryFirmnessBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_berry_firmness() {
    let state = get_app_state();
    let response: BerryFirmnessBulkResponse = test_get("/berry-firmness").await.unwrap();

    let size = state.berry_firmness.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_berry_firmness() {
    let response: BerryFirmnessBulkResponse = test_get("/berry-firmness?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "very-soft");
    assert_eq!(names.get(9), Some("Very Soft".to_string()));
}
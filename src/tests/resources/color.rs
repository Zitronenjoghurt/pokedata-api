use crate::get_app_state;
use crate::models::bulk_response::ColorBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_colors() {
    let state = get_app_state();
    let response: ColorBulkResponse = test_get("/color").await.unwrap();

    let size = state.colors.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_color() {
    let response: ColorBulkResponse = test_get("/color?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "black".to_string());
    assert_eq!(names.get(9), Some("Black".to_string()))
}
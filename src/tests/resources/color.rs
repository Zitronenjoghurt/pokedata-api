use crate::get_app_state;
use crate::models::bulk_response::ColorBulkResponse;
use crate::tests::resources::get_test;

#[tokio::test]
async fn test_get_all_colors() {
    let state = get_app_state();
    let response: ColorBulkResponse = get_test("/color").await.unwrap();

    let colors_size = state.colors.len();
    assert_eq!(response.count, colors_size);
}

#[tokio::test]
async fn test_get_specific_color() {
    let response: ColorBulkResponse = get_test("/color?ids=1").await.unwrap();
    let black = response.results.get(0).cloned().unwrap_or_default();

    let names = black.names.unwrap_or_default();
    assert_eq!(black.id, 1);
    assert_eq!(names.get(9), Some("Black".to_string()))
}
use crate::models::bulk_response::ColorBulkResponse;
use crate::tests::resources::create_test_server;

#[tokio::test]
async fn test_get_all_colors() {
    let server = create_test_server();

    let test_response = server.get("/color").await;
    assert!(test_response.status_code().is_success());

    let response: ColorBulkResponse = test_response.json();

    assert_eq!(response.count, 10);
}
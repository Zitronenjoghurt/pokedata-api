use crate::models::message_response::MessageResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_ping() {
    let response: MessageResponse = test_get("/").await.unwrap();
    assert_eq!(response.message, "Pong");
}

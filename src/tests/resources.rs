use crate::build_app;
use axum_test::TestServer;

mod color;

pub fn create_test_server() -> TestServer {
    let app = build_app();
    TestServer::new(app).unwrap()
}
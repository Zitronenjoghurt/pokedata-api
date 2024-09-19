use crate::build_app;
use axum_test::TestServer;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;

mod color;

static TEST_SERVER: Lazy<TestServer> = Lazy::new(|| {
    let app = build_app();
    TestServer::new(app).unwrap()
});

pub fn get_test_server() -> &'static TestServer {
    &TEST_SERVER
}

pub async fn get_test<T>(url: &str) -> Result<T, Box<dyn std::error::Error>>
where
    T: DeserializeOwned,
{
    let response = get_test_server().get(url).await;

    if !response.status_code().is_success() {
        return Err(format!("Request failed with status: {}", response.status_code()).into());
    }

    let data: T = response.json::<T>();
    Ok(data)
}
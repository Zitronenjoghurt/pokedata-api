use crate::build_app;
use axum::http::StatusCode;
use axum_test::TestServer;
use once_cell::sync::Lazy;
use serde::de::DeserializeOwned;

mod color;
mod generation;
mod growth_rate;
mod habitat;
mod language;
mod ping;
mod pokemon_move_target;
mod pokemon_type;
mod pokemon_type_efficacy;
mod region;
mod shape;
mod stat;
mod version;
mod version_group;
mod pokemon_move_flag;
mod pokemon_move_ailment;
mod pokemon_move_category;
mod tcg_card;
mod pokemon_move_effect;
mod ability;
mod berry;
mod berry_firmness;
mod berry_flavor;
mod egg_group;

static TEST_SERVER: Lazy<TestServer> = Lazy::new(|| {
    let app = build_app();
    TestServer::new(app).unwrap()
});

pub fn get_test_server() -> &'static TestServer {
    &TEST_SERVER
}

pub async fn test_get<T>(url: &str) -> Result<T, Box<dyn std::error::Error>>
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

pub async fn test_get_code(url: &str, code: StatusCode) {
    let response = get_test_server().get(url).await;
    assert_eq!(response.status_code(), code);
}
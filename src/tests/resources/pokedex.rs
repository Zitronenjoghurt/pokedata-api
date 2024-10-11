use crate::get_app_state;
use crate::models::bulk_response::PokedexBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_pokedexes() {
    let state = get_app_state();
    let response: PokedexBulkResponse = test_get("/pokedex").await.unwrap();

    let size = state.pokedexes.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_pokedex() {
    let response: PokedexBulkResponse = test_get("/pokedex?ids=2").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    let descriptions = test_entity.descriptions.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 2);
    assert_eq!(test_entity.region_id, Some(1));
    assert_eq!(test_entity.identifier, "kanto");
    assert!(test_entity.is_main_series);
    assert!(test_entity.version_group_ids.contains(&1));
    assert!(test_entity.version_group_ids.contains(&2));
    assert!(test_entity.version_group_ids.contains(&7));
    assert_eq!(test_entity.version_group_ids.len(), 3);
    assert_eq!(names.get(9), Some("Kanto".to_string()));
    assert_eq!(descriptions.get(9), Some("Red/Blue/Yellow Kanto dex".to_string()));
}
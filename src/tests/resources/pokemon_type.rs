use crate::get_app_state;
use crate::models::bulk_response::PokemonTypeBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_type() {
    let state = get_app_state();
    let response: PokemonTypeBulkResponse = test_get("/pokemon-type").await.unwrap();

    let size = state.types.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_type() {
    let response: PokemonTypeBulkResponse = test_get("/pokemon-type?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "normal");
    assert_eq!(test_entity.generation_id, 1);
    assert!(test_entity.is_major_type);
    assert_eq!(test_entity.damage_class_id, Some(2));
    assert_eq!(names.get(9), Some("Normal".to_string()))
}
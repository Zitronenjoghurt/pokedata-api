use crate::get_app_state;
use crate::models::bulk_response::ShapeBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_shapes() {
    let state = get_app_state();
    let response: ShapeBulkResponse = test_get("/shape").await.unwrap();

    let size = state.shapes.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_shape() {
    let response: ShapeBulkResponse = test_get("/shape?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    let awesome_names = test_entity.awesome_names.unwrap_or_default();
    let descriptions = test_entity.descriptions.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "ball");
    assert_eq!(names.get(9), Some("Ball".to_string()));
    assert_eq!(awesome_names.get(9), Some("Pomaceous".to_string()));
    assert_eq!(descriptions.get(9), Some("Pokémon consisting of only a head".to_string()));
}
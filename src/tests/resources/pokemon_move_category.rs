use crate::get_app_state;
use crate::models::bulk_response::PokemonMoveCategoryBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_move_categories() {
    let state = get_app_state();
    let response: PokemonMoveCategoryBulkResponse = test_get("/move-category").await.unwrap();

    let size = state.move_categories.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_move_category() {
    let response: PokemonMoveCategoryBulkResponse = test_get("/move-category?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let descriptions = test_entity.descriptions.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "ailment");
    assert_eq!(descriptions.get(9), Some("No damage; inflicts status ailment".to_string()));
}
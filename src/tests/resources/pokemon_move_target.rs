use crate::get_app_state;
use crate::models::bulk_response::PokemonMoveTargetBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_move_targets() {
    let state = get_app_state();
    let response: PokemonMoveTargetBulkResponse = test_get("/move-target").await.unwrap();

    let size = state.move_targets.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_move_target() {
    let response: PokemonMoveTargetBulkResponse = test_get("/move-target?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    let descriptions = test_entity.descriptions.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "specific-move");
    assert_eq!(names.get(9), Some("Specific move".to_string()));
    assert_eq!(descriptions.get(9), Some("One specific move.  How this move is chosen depends upon on the move being used.".to_string()));
}
use crate::get_app_state;
use crate::models::bulk_response::PokemonMoveFlagBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_move_flags() {
    let state = get_app_state();
    let response: PokemonMoveFlagBulkResponse = test_get("/move-flag").await.unwrap();

    let size = state.move_flags.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_move_flag() {
    let response: PokemonMoveFlagBulkResponse = test_get("/move-flag?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    let descriptions = test_entity.descriptions.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "contact");
    assert_eq!(names.get(9), Some("Makes contact".to_string()));
    assert_eq!(descriptions.get(9), Some("User touches the target.  This triggers some abilities (e.g., []{ability:static}) and items (e.g., []{item:sticky-barb}).".to_string()));
}
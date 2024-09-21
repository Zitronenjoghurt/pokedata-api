use crate::get_app_state;
use crate::models::bulk_response::StatBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_stats() {
    let state = get_app_state();
    let response: StatBulkResponse = test_get("/stat").await.unwrap();

    let size = state.stats.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_stat() {
    let response: StatBulkResponse = test_get("/stat?ids=2").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 2);
    assert_eq!(test_entity.identifier, "attack");
    assert_eq!(test_entity.damage_class_id, Some(2));
    assert!(!test_entity.is_battle_only);
    assert_eq!(test_entity.game_index, Some(2));
    assert_eq!(names.get(9), Some("Attack".to_string()));
}
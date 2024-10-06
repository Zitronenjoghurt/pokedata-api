use crate::get_app_state;
use crate::models::bulk_response::EggGroupBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_egg_groups() {
    let state = get_app_state();
    let response: EggGroupBulkResponse = test_get("/egg-group").await.unwrap();

    let size = state.egg_groups.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_egg_group() {
    let response: EggGroupBulkResponse = test_get("/egg-group?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "monster".to_string());
    assert_eq!(names.get(9), Some("Monster".to_string()))
}
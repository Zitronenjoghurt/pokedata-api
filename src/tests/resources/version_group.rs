use crate::get_app_state;
use crate::models::bulk_response::VersionGroupBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_version_groups() {
    let state = get_app_state();
    let response: VersionGroupBulkResponse = test_get("/version-group").await.unwrap();

    let size = state.version_groups.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_version_group() {
    let response: VersionGroupBulkResponse = test_get("/version-group?ids=1").await.unwrap();
    let mut test_entity = response.results.get(0).cloned().unwrap_or_default();
    test_entity.version_ids.sort();
    test_entity.region_ids.sort();
    test_entity.move_method_ids.sort();

    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "red-blue");
    assert_eq!(test_entity.generation_id, 1);
    assert_eq!(test_entity.order, 1);
    assert_eq!(test_entity.version_ids, vec![1, 2]);
    assert_eq!(test_entity.region_ids, vec![1]);
    assert_eq!(test_entity.move_method_ids, vec![1, 4, 5]);
}
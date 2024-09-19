use crate::get_app_state;
use crate::models::bulk_response::VersionBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_versions() {
    let state = get_app_state();
    let response: VersionBulkResponse = test_get("/version").await.unwrap();

    let size = state.versions.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_version() {
    let response: VersionBulkResponse = test_get("/version?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "red");
    assert_eq!(test_entity.version_group_id, 1);
    assert_eq!(names.get(9), Some("Red".to_string()))
}
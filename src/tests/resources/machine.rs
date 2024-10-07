use crate::get_app_state;
use crate::models::bulk_response::MachineBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_machines() {
    let state = get_app_state();
    let response: MachineBulkResponse = test_get("/machine").await.unwrap();

    let size = state.machines.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_machine() {
    let response: MachineBulkResponse = test_get("/machine?ids=0").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 0);
    assert_eq!(test_entity.machine_number, 0);
    assert_eq!(test_entity.version_group_id, 20);
    assert_eq!(test_entity.item_id, 1288);
    assert_eq!(test_entity.move_id, 5);
}
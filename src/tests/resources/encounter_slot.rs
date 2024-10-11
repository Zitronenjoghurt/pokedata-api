use crate::get_app_state;
use crate::models::bulk_response::EncounterSlotBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_encounter_slots() {
    let state = get_app_state();
    let response: EncounterSlotBulkResponse = test_get("/encounter-slot").await.unwrap();

    let size = state.encounter_slots.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_encounter_slot() {
    let response: EncounterSlotBulkResponse = test_get("/encounter-slot?ids=1").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.version_group_id, 8);
    assert_eq!(test_entity.encounter_method_id, 1);
    assert_eq!(test_entity.slot, Some(1));
    assert_eq!(test_entity.rarity, 20);
}
use crate::models::type_effectiveness::{AllTypeEffectivenessResponse, TypeEffectivenessResponse};
use crate::tests::resources::{test_get, test_get_code};
use axum::http::StatusCode;

#[tokio::test]
async fn test_get_type_effectiveness() {
    test_get_code("/type-efficacy?defending_ids=2", StatusCode::BAD_REQUEST).await;

    // Psychic vs. Fighting+Poison = 4x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=14&defending_ids=2,4").await.unwrap();
    assert_eq!(response.0, Some(400));

    // Dragon vs. Fairy+Ground = 0x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=16&defending_ids=5,18").await.unwrap();
    assert_eq!(response.0, Some(0));
}

#[tokio::test]
async fn test_get_all_effectiveness() {
    let response: AllTypeEffectivenessResponse = test_get("/type-efficacy/all?defending_ids=10,13&generation_id=9").await.unwrap();
    let map = response.0;

    assert_eq!(map.get(&1), Some(&100));
    assert_eq!(map.get(&2), Some(&100));
    assert_eq!(map.get(&3), Some(&50));
    assert_eq!(map.get(&4), Some(&100));
    assert_eq!(map.get(&5), Some(&400));
    assert_eq!(map.get(&6), Some(&200));
    assert_eq!(map.get(&7), Some(&50));
    assert_eq!(map.get(&8), Some(&100));
    assert_eq!(map.get(&9), Some(&25));
    assert_eq!(map.get(&10), Some(&50));
    assert_eq!(map.get(&11), Some(&200));
    assert_eq!(map.get(&12), Some(&50));
    assert_eq!(map.get(&13), Some(&50));
    assert_eq!(map.get(&14), Some(&100));
    assert_eq!(map.get(&15), Some(&50));
    assert_eq!(map.get(&16), Some(&100));
    assert_eq!(map.get(&17), Some(&100));
    assert_eq!(map.get(&18), Some(&50));
}

#[tokio::test]
async fn test_get_generation_specific_type_effectiveness() {
    // Ghost vs. Psychic in Gen 1 = 0x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=14&generation_id=1").await.unwrap();
    assert_eq!(response.0, Some(0));

    // Ghost vs. Psychic in Gen 2 = 2x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=14&generation_id=2").await.unwrap();
    assert_eq!(response.0, Some(200));

    // Ghost vs. Psychic in Gen 6 = 2x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=14&generation_id=6").await.unwrap();
    assert_eq!(response.0, Some(200));

    // Ghost vs. Psychic in latest gen = 2x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=14").await.unwrap();
    assert_eq!(response.0, Some(200));

    // Ghost vs. Steel in gen 1 = 0.5x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=9&generation_id=1").await.unwrap();
    assert_eq!(response.0, Some(50));

    // Ghost vs. Steel in gen 5 = 0.5x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=9&generation_id=5").await.unwrap();
    assert_eq!(response.0, Some(50));

    // Ghost vs. Steel in gen 6 = 1x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=9&generation_id=6").await.unwrap();
    assert_eq!(response.0, Some(100));

    // Ghost vs. Steel in latest gen = 1x
    let response: TypeEffectivenessResponse = test_get("/type-efficacy?attacking_id=8&defending_ids=9").await.unwrap();
    assert_eq!(response.0, Some(100));
}
use crate::get_app_state;
use crate::models::bulk_response::GrowthRateBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_growth_rate() {
    let state = get_app_state();
    let response: GrowthRateBulkResponse = test_get("/growth-rate").await.unwrap();

    let size = state.growth_rates.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_growth_rate() {
    let response: GrowthRateBulkResponse = test_get("/growth-rate?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "slow");
    assert_eq!(test_entity.formula, "\\frac{5x^3}{4}");
    assert_eq!(names.get(9), Some("slow".to_string()))
}
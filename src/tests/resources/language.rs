use crate::get_app_state;
use crate::models::bulk_response::LanguageBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_languages() {
    let state = get_app_state();
    let response: LanguageBulkResponse = test_get("/language").await.unwrap();

    let size = state.languages.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_language() {
    let response: LanguageBulkResponse = test_get("/language?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.iso639, "ja");
    assert_eq!(test_entity.iso3166, "jp");
    assert!(test_entity.official);
    assert_eq!(test_entity.order, 1);
    assert_eq!(names.get(9), Some("Japanese".to_string()))
}
use crate::get_app_state;
use crate::models::bulk_response::TcgCardBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_tcg_cards() {
    let state = get_app_state();
    let response: TcgCardBulkResponse = test_get("/tcg-card").await.unwrap();

    let size = state.tcg_cards.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

// ! IDs are not consitently assigned to the same dataset
// ToDo: Test specific entity by card identifier not by card id
//#[tokio::test]
//async fn test_get_specific_tcg_card() {
//    let response: TcgCardBulkResponse = test_get("/tcg-card?ids=1").await.unwrap();
//    let test_entity = response.results.get(0).cloned().unwrap_or_default();
//
//
//    assert_eq!(response.count, 1);
//    assert_eq!(test_entity.id, 1);
//    assert_eq!(test_entity.identifier, "swsh8-2");
//    assert_eq!(test_entity.name, "Metapod");
//    assert_eq!(test_entity.super_type, "Pokémon");
//    assert_eq!(test_entity.sub_types, vec!["Stage 1"]);
//    assert_eq!(test_entity.level, None);
//    assert_eq!(test_entity.hp, Some(80));
//    assert_eq!(test_entity.types, vec!["Grass"]);
//    assert_eq!(test_entity.evolves_from, Some("Caterpie".to_string()));
//    assert_eq!(test_entity.evolves_to, vec!["Butterfree"]);
//    assert!(test_entity.rules.is_empty());
//    assert_eq!(test_entity.ancient_trait, None);
//}
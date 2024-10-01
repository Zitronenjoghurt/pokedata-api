use crate::get_app_state;
use crate::models::bulk_response::TcgCardBulkResponse;
use crate::tests::resources::test_get;
use pokedata_api_entities::api::tcg_card::TcgCard;

#[tokio::test]
async fn test_get_all_tcg_cards() {
    let state = get_app_state();
    let response: TcgCardBulkResponse = test_get("/tcg-card").await.unwrap();

    let size = state.tcg_cards.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}
#[tokio::test]
async fn test_get_specific_tcg_card() {
    let test_entity: TcgCard = test_get("/tcg-card?identifier=swsh8-2").await.unwrap();

    assert_eq!(test_entity.identifier, "swsh8-2");
    assert_eq!(test_entity.name, "Metapod");
    assert_eq!(test_entity.super_type, "Pokémon");
    assert_eq!(test_entity.sub_types, vec!["Stage 1"]);
    assert_eq!(test_entity.level, None);
    assert_eq!(test_entity.hp, Some(80));
    assert_eq!(test_entity.types, vec!["Grass"]);
    assert_eq!(test_entity.evolves_from, Some("Caterpie".to_string()));
    assert_eq!(test_entity.evolves_to, vec!["Butterfree"]);
    assert!(test_entity.rules.is_empty());
    assert_eq!(test_entity.ancient_trait, None);
}
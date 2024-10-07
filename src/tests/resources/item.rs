use crate::get_app_state;
use crate::models::bulk_response::ItemBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_items() {
    let state = get_app_state();
    let response: ItemBulkResponse = test_get("/item").await.unwrap();

    let size = state.items.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_item() {
    let response: ItemBulkResponse = test_get("/item?ids=126").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    let short_effects = test_entity.short_effects.unwrap_or_default();
    let effects = test_entity.effects.unwrap_or_default();
    assert_eq!(test_entity.id, 126);
    assert_eq!(test_entity.identifier, "cheri-berry");
    assert_eq!(test_entity.category_id, 3);
    assert_eq!(test_entity.cost, 80);
    assert_eq!(test_entity.fling_power, Some(10));
    assert_eq!(test_entity.fling_effect_id, Some(3));
    assert_eq!(test_entity.flag_ids, vec![7]);
    assert!(test_entity.game_indices.contains(&133));
    assert!(test_entity.game_indices.contains(&149));
    assert_eq!(test_entity.game_indices.len(), 2);
    assert!(test_entity.generation_ids.contains(&3));
    assert!(test_entity.generation_ids.contains(&4));
    assert!(test_entity.generation_ids.contains(&5));
    assert!(test_entity.generation_ids.contains(&6));
    assert!(test_entity.generation_ids.contains(&7));
    assert!(test_entity.generation_ids.contains(&8));
    assert_eq!(test_entity.generation_ids.len(), 6);
    assert_eq!(names.get(9), Some("Cheri Berry".to_string()));
    assert_eq!(short_effects.get(9), Some("Held: Consumed when paralyzed to cure paralysis.".to_string()));
    assert_eq!(effects.get(9), Some("Held in battle\n:   When the holder is [paralyzed]{mechanic:paralysis}, it consumes this item to cure the paralysis.\n\nUsed on a party Pokémon\n:   Cures [paralysis]{mechanic:paralysis}.".to_string()));

    let flavor_texts = test_entity.flavor_texts.unwrap_or_default();
    let localized_value_sample = flavor_texts.0.get(&5).cloned().unwrap_or_default();
    let flavor_text_sample = localized_value_sample.0.get(&9).cloned().unwrap_or_default();
    assert_eq!(flavor_text_sample, "A hold item that\nheals paralysis\nin battle.");
}
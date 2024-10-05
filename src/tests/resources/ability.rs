use crate::get_app_state;
use crate::models::bulk_response::AbilityBulkResponse;
use crate::tests::resources::test_get;

#[tokio::test]
async fn test_get_all_abilities() {
    let state = get_app_state();
    let response: AbilityBulkResponse = test_get("/ability").await.unwrap();

    let size = state.abilities.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_ability() {
    let response: AbilityBulkResponse = test_get("/ability?ids=1").await.unwrap();
    let test_entity = response.results.get(0).cloned().unwrap_or_default();

    let names = test_entity.names.unwrap_or_default();
    let effects = test_entity.effects.unwrap_or_default();
    let short_effects = test_entity.short_effects.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 1);
    assert_eq!(test_entity.identifier, "stench".to_string());
    assert_eq!(names.get(9), Some("Stench".to_string()));
    assert_eq!(effects.get(9), Some("This Pokémon's damaging moves have a 10% chance to make the target [flinch]{mechanic:flinch} with each hit if they do not already cause flinching as a secondary effect.\n\nThis ability does not stack with a held item.\n\nOverworld: The wild encounter rate is halved while this Pokémon is first in the party.".to_string()));
    assert_eq!(short_effects.get(9), Some("Has a 10% chance of making target Pokémon [flinch]{mechanic:flinch} with each hit.".to_string()));

    let version_grouped_localized_values = test_entity.flavor_texts.unwrap_or_default();
    let localized_values = version_grouped_localized_values.0.get(&5).cloned().unwrap_or_default();
    let sample_text = localized_values.get(9).unwrap_or_default();
    assert_eq!(sample_text, "Helps repel wild POKéMON.");
}
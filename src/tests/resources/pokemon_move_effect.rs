use crate::get_app_state;
use crate::models::bulk_response::PokemonMoveEffectBulkResponse;
use crate::tests::resources::test_get;
use pokedata_api_entities::api::localized_values::LocalizedValues;
use pokedata_api_entities::api::pokemon_move_effect_changelog_entry::PokemonMoveEffectChangelogEntry;
use std::collections::HashMap;

#[tokio::test]
async fn test_get_all_move_effects() {
    let state = get_app_state();
    let response: PokemonMoveEffectBulkResponse = test_get("/move-effect").await.unwrap();

    let size = state.move_effects.len();
    assert_eq!(response.count, size);
    assert_eq!(response.results.len(), size);
}

#[tokio::test]
async fn test_get_specific_move_effect() {
    let response: PokemonMoveEffectBulkResponse = test_get("/move-effect?ids=8").await.unwrap();
    let test_entity = response.results.first().cloned().unwrap_or_default();

    let effect = test_entity.effect.unwrap_or_default();
    let short_effect = test_entity.short_effect.unwrap_or_default();
    assert_eq!(response.count, 1);
    assert_eq!(test_entity.id, 8);
    assert_eq!(effect.get(9), Some("User [faint]{mechanic:faint}s, even if the attack [fail]{mechanic:fail}s or [miss]{mechanic:miss}es.  Inflicts [regular damage]{mechanic:regular-damage}.".to_string()));
    assert_eq!(short_effect.get(9), Some("User faints.".to_string()));
    assert_eq!(test_entity.move_ids, vec![120, 153]);

    let mut localized_values: HashMap<i32, String> = HashMap::new();
    localized_values.insert(9, "Halves the target's [Defense]{mechanic:defense} for damage calculation, which is similar to doubling the attack's [power]{mechanic:power}.".to_string());

    let mut changelog_entries: HashMap<i32, PokemonMoveEffectChangelogEntry> = HashMap::new();
    changelog_entries.insert(11, PokemonMoveEffectChangelogEntry {
        id: 1,
        effects: Some(LocalizedValues(localized_values)),
    });
    assert_eq!(test_entity.changelogs, changelog_entries);
}
use pokedata_api_entities::api::pokemon_move::PokemonMove;
use pokedata_api_entities::api::pokemon_move_effect::PokemonMoveEffect;
use std::collections::HashMap;

pub fn enrich_move_effects_with_move_ids(
    moves: &HashMap<i32, PokemonMove>,
    move_effects: &mut HashMap<i32, PokemonMoveEffect>,
)
{
    for pokemon_move in moves.values() {
        let Some(effect_id) = pokemon_move.effect_id else { continue };
        let Some(effect) = move_effects.get_mut(&effect_id) else { continue };
        effect.move_ids.push(pokemon_move.id);
    }
}
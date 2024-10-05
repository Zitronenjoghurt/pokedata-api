use crate::enrich_data::card_ids_for_tcg_sets::enrich_tcg_sets_with_card_ids;
use crate::enrich_data::move_ids_for_move_effects::enrich_move_effects_with_move_ids;
use crate::enrich_data::set_id_for_tcg_cards::enrich_tcg_cards_with_set_id;
use crate::enrich_data::tcg_ids_for_species::enrich_species_with_tcg_ids;
use crate::search_index::build_search_indices;
use crate::sprites::load_sprite_index;
use pokedata_api_entities::api::pokemon_type::get_major_type_ids;
use pokedata_api_entities::app_state::AppState;
use pokedata_api_entities::traits::into_id_map::IntoIdMap;
use pokedata_api_parsing::csv::evolution_chains::EvolutionChainConversionData;
use pokedata_api_parsing::csv::move_damage_classes::PokemonMoveDamageClassConversionData;
use pokedata_api_parsing::csv::move_effects::PokemonMoveEffectConversionData;
use pokedata_api_parsing::csv::move_flags::PokemonMoveFlagConversionData;
use pokedata_api_parsing::csv::move_targets::PokemonMoveTargetConversionData;
use pokedata_api_parsing::csv::moves::PokemonMoveConversionData;
use pokedata_api_parsing::csv::pokemon::PokemonConversionData;
use pokedata_api_parsing::csv::pokemon_shapes::PokemonShapesConversionData;
use pokedata_api_parsing::csv::type_efficacy::build_efficacies_by_generation;
use pokedata_api_parsing::csv::*;
use pokedata_api_parsing::csv_entity::CSVEntity;
use pokedata_api_parsing::pokemon_tcg::{load_tcg_cards, load_tcg_sets};
use pokedata_api_parsing::traits::api_csv_entity::ApiCSVEntity;
use pokedata_api_parsing::traits::into_localized_values_map::IntoLocalizedValuesMap;
use std::path::PathBuf;
use std::sync::Arc;

pub fn create_app_state(csv_path: &PathBuf) -> AppState {
    let mut tcg_cards = load_tcg_cards();
    let mut tcg_sets = load_tcg_sets();

    let ailment_names = move_meta_ailment_names::MoveMetaAilmentNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let color_names = pokemon_color_names::PokemonColorNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let generation_names = generation_names::GenerationNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let growth_rate_names = growth_rate_prose::GrowthRateProseCSV::load(csv_path).unwrap().into_localized_values_map();
    let habitat_names = pokemon_habitat_names::PokemonHabitatNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let language_names = language_names::LanguageNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let move_category_descriptions = move_meta_category_prose::MoveMetaCategoryProseCSV::load(csv_path).unwrap().into_localized_values_map();
    let region_names = region_names::RegionNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let stat_names = stat_names::StatNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let type_names = type_names::TypeNamesCSV::load(csv_path).unwrap().into_localized_values_map();
    let version_names = version_names::VersionNamesCSV::load(csv_path).unwrap().into_localized_values_map();

    let sprite_index = Arc::new(load_sprite_index());
    let moves_data = PokemonMoveConversionData::load(csv_path);
    let move_damage_classes_data = PokemonMoveDamageClassConversionData::load(csv_path);
    let move_effects_data = PokemonMoveEffectConversionData::load(csv_path);
    let move_flags_data = PokemonMoveFlagConversionData::load(csv_path);
    let move_targets_data = PokemonMoveTargetConversionData::load(csv_path);
    let pokemon_data = PokemonConversionData::load(csv_path, sprite_index);
    let pokemon_shapes_data = PokemonShapesConversionData::load(csv_path);

    let abilities = abilities::AbilitiesCSV::load_and_convert(csv_path, &()).unwrap().into_id_map();
    let colors = pokemon_colors::PokemonColorsCSV::load_and_convert(csv_path, &color_names).unwrap().into_id_map();
    let evolutions = pokemon_evolution::PokemonEvolutionCSV::load_and_convert(csv_path, &()).unwrap().into_id_map();
    let generations = generations::GenerationsCSV::load_and_convert(csv_path, &generation_names).unwrap().into_id_map();
    let growth_rates = growth_rates::GrowthRatesCSV::load_and_convert(csv_path, &growth_rate_names).unwrap().into_id_map();
    let habitats = pokemon_habitats::PokemonHabitatsCSV::load_and_convert(csv_path, &habitat_names).unwrap().into_id_map();
    let languages = languages::LanguagesCSV::load_and_convert(csv_path, &language_names).unwrap().into_id_map();
    let moves = moves::MovesCSV::load_and_convert(csv_path, &moves_data).unwrap().into_id_map();
    let move_ailments = move_meta_ailments::MoveMetaAilmentsCSV::load_and_convert(csv_path, &ailment_names).unwrap().into_id_map();
    let move_categories = move_meta_categories::MoveMetaCategoriesCSV::load_and_convert(csv_path, &move_category_descriptions).unwrap().into_id_map();
    let move_damage_classes = move_damage_classes::MoveDamageClassesCSV::load_and_convert(csv_path, &move_damage_classes_data).unwrap().into_id_map();
    let mut move_effects = move_effects::MoveEffectsCSV::load_and_convert(csv_path, &move_effects_data).unwrap().into_id_map();
    let move_flags = move_flags::MoveFlagsCSV::load_and_convert(csv_path, &move_flags_data).unwrap().into_id_map();
    let move_targets = move_targets::MoveTargetsCSV::load_and_convert(csv_path, &move_targets_data).unwrap().into_id_map();
    let pokemon = pokemon::PokemonCSV::load_and_convert(csv_path, &pokemon_data).unwrap().into_id_map();
    let regions = regions::RegionsCSV::load_and_convert(csv_path, &region_names).unwrap().into_id_map();
    let shapes = pokemon_shapes::PokemonShapesCSV::load_and_convert(csv_path, &pokemon_shapes_data).unwrap().into_id_map();
    let stats = stats::StatsCSV::load_and_convert(csv_path, &stat_names).unwrap().into_id_map();
    let types = types::TypesCSV::load_and_convert(csv_path, &type_names).unwrap().into_id_map();
    let versions = versions::VersionsCSV::load_and_convert(csv_path, &version_names).unwrap().into_id_map();

    let latest_generation = generations.keys().max().copied().unwrap_or(1);
    let version_group_data = version_groups::VersionGroupConversionData::load(csv_path, &versions);
    let species_data = pokemon_species::PokemonSpeciesConversionData::load(csv_path, &pokemon);

    let mut species = pokemon_species::PokemonSpeciesCSV::load_and_convert(csv_path, &species_data).unwrap().into_id_map();
    let type_efficacies = build_efficacies_by_generation(csv_path, latest_generation).unwrap();
    let version_groups = version_groups::VersionGroupsCSV::load_and_convert(csv_path, &version_group_data).unwrap().into_id_map();

    let evolution_chain_data = EvolutionChainConversionData::load(&evolutions, &species);
    let evolution_chains = evolution_chains::EvolutionChainsCSV::load_and_convert(csv_path, &evolution_chain_data).unwrap().into_id_map();

    let major_type_ids = get_major_type_ids(types.values().cloned().collect());

    let search_indices = build_search_indices(&tcg_cards, &tcg_sets);

    enrich_move_effects_with_move_ids(&moves, &mut move_effects);
    enrich_tcg_cards_with_set_id(&mut tcg_cards, &search_indices.set_identifier_to_set_id);
    enrich_tcg_sets_with_card_ids(&tcg_cards, &mut tcg_sets, &search_indices.set_identifier_to_set_id);

    enrich_species_with_tcg_ids(&tcg_cards, &mut species);

    AppState {
        abilities,
        colors,
        evolutions,
        evolution_chains,
        generations,
        growth_rates,
        habitats,
        languages,
        moves,
        move_ailments,
        move_categories,
        move_damage_classes,
        move_effects,
        move_flags,
        move_targets,
        pokemon,
        regions,
        shapes,
        species,
        stats,
        tcg_cards,
        tcg_sets,
        types,
        type_efficacies,
        versions,
        version_groups,
        latest_generation,
        major_type_ids,
        search_indices,
    }
}
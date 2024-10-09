use crate::csv::abilities::AbilitiesCSV;
use crate::csv::ability_changelog::AbilityChangelogCSV;
use crate::csv::ability_changelog_prose::AbilityChangelogProseCSV;
use crate::csv::ability_flavor_text::AbilityFlavorTextCSV;
use crate::csv::ability_names::AbilityNamesCSV;
use crate::csv::ability_prose::{AbilityEffectsCSV, AbilityShortEffectsCSV};
use crate::csv::berries::BerriesCSV;
use crate::csv::berry_firmness::BerryFirmnessCSV;
use crate::csv::berry_firmness_names::BerryFirmnessNamesCSV;
use crate::csv::berry_flavors::BerryFlavorPotenciesCSV;
use crate::csv::contest_type_names::BerryFlavorNamesCSV;
use crate::csv::egg_group_prose::EggGroupProseCSV;
use crate::csv::egg_groups::EggGroupsCSV;
use crate::csv::evolution_chains::EvolutionChainsCSV;
use crate::csv::evolution_trigger_prose::EvolutionTriggerProseCSV;
use crate::csv::evolution_triggers::EvolutionTriggersCSV;
use crate::csv::generation_names::GenerationNamesCSV;
use crate::csv::generations::GenerationsCSV;
use crate::csv::growth_rate_prose::GrowthRateProseCSV;
use crate::csv::growth_rates::GrowthRatesCSV;
use crate::csv::item_categories::ItemCategoriesCSV;
use crate::csv::item_category_prose::ItemCategoryProseCSV;
use crate::csv::item_flag_map::ItemFlagMapCSV;
use crate::csv::item_flag_prose::{ItemFlagDescriptionsCSV, ItemFlagNamesCSV};
use crate::csv::item_flags::ItemFlagsCSV;
use crate::csv::item_flavor_text::ItemFlavorTextCSV;
use crate::csv::item_game_indices::{ItemGameIndicesCSV, ItemGenerationIdsCSV};
use crate::csv::item_names::ItemNamesCSV;
use crate::csv::item_pocket_names::ItemPocketNamesCSV;
use crate::csv::item_pockets::ItemPocketsCSV;
use crate::csv::items::ItemsCSV;
use crate::csv::items_prose::{ItemEffectsCSV, ItemShortEffectsCSV};
use crate::csv::language_names::LanguageNamesCSV;
use crate::csv::languages::LanguagesCSV;
use crate::csv::location_area_encounter_rates::LocationAreaEncounterRatesCSV;
use crate::csv::location_area_prose::LocationAreaProseCSV;
use crate::csv::location_areas::LocationAreasCSV;
use crate::csv::location_game_indices::{LocationGameIndicesCSV, LocationGenerationIdsCSV};
use crate::csv::location_names::{LocationNamesCSV, LocationSubtitlesCSV};
use crate::csv::locations::LocationsCSV;
use crate::csv::machines::MachinesCSV;
use crate::csv::move_changelog::MoveChangelogCSV;
use crate::csv::move_damage_class_prose::{MoveDamageClassDescriptionsCSV, MoveDamageClassNamesCSV};
use crate::csv::move_damage_classes::MoveDamageClassesCSV;
use crate::csv::move_effect_changelog::MoveEffectChangelogCSV;
use crate::csv::move_effect_changelog_prose::MoveEffectChangelogProseCSV;
use crate::csv::move_effect_prose::{MoveEffectEffectsCSV, MoveEffectShortEffectsCSV};
use crate::csv::move_effects::MoveEffectsCSV;
use crate::csv::move_flag_map::MoveFlagMapCSV;
use crate::csv::move_flag_prose::{MoveFlagDescriptionsCSV, MoveFlagNamesCSV};
use crate::csv::move_flags::MoveFlagsCSV;
use crate::csv::move_flavor_text::MoveFlavorTextCSV;
use crate::csv::move_meta::MoveMetaCSV;
use crate::csv::move_meta_ailment_names::MoveMetaAilmentNamesCSV;
use crate::csv::move_meta_ailments::MoveMetaAilmentsCSV;
use crate::csv::move_meta_categories::MoveMetaCategoriesCSV;
use crate::csv::move_meta_category_prose::MoveMetaCategoryProseCSV;
use crate::csv::move_meta_stat_changes::MoveMetaStatChangesCSV;
use crate::csv::move_names::MoveNamesCSV;
use crate::csv::move_target_prose::{MoveTargetDescriptionsCSV, MoveTargetNamesCSV};
use crate::csv::move_targets::MoveTargetsCSV;
use crate::csv::moves::MovesCSV;
use crate::csv::pokemon::PokemonCSV;
use crate::csv::pokemon_color_names::PokemonColorNamesCSV;
use crate::csv::pokemon_colors::PokemonColorsCSV;
use crate::csv::pokemon_evolution::PokemonEvolutionCSV;
use crate::csv::pokemon_game_indices::PokemonVersionIdsCSV;
use crate::csv::pokemon_habitat_names::PokemonHabitatNamesCSV;
use crate::csv::pokemon_habitats::PokemonHabitatsCSV;
use crate::csv::pokemon_shape_prose::{PokemonShapeAwesomeNamesCSV, PokemonShapeDescriptionsCSV, PokemonShapeNamesCSV};
use crate::csv::pokemon_shapes::PokemonShapesCSV;
use crate::csv::pokemon_species::PokemonSpeciesCSV;
use crate::csv::pokemon_species_flavor_text::PokemonSpeciesFlavorTextCSV;
use crate::csv::pokemon_species_names::PokemonSpeciesNamesCSV;
use crate::csv::pokemon_stats::PokemonStatsCSV;
use crate::csv::pokemon_types::PokemonTypesCSV;
use crate::csv::pokemon_types_past::PokemonTypesPastCSV;
use crate::csv::region_names::RegionNamesCSV;
use crate::csv::regions::RegionsCSV;
use crate::csv::stat_names::StatNamesCSV;
use crate::csv::stats::StatsCSV;
use crate::csv::type_efficacy::TypeEfficacyCSV;
use crate::csv::type_efficacy_past::TypeEfficacyPastCSV;
use crate::csv::type_names::TypeNamesCSV;
use crate::csv::types::TypesCSV;
use crate::csv::version_group_pokemon_move_methods::VersionGroupPokemonMoveMethodsCSV;
use crate::csv::version_group_regions::VersionGroupRegionsCSV;
use crate::csv::version_groups::VersionGroupsCSV;
use crate::csv::version_names::VersionNamesCSV;
use crate::csv::versions::VersionsCSV;
use csv::Reader;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct CSVEntityMetaData {
    pub file_name: String,
    pub download_url: String,
    pub is_downloadable: bool,
}

pub trait CSVEntity: DeserializeOwned {
    fn file_name() -> &'static str;

    fn base_download_url() -> &'static str {
        "https://raw.githubusercontent.com/PokeAPI/pokeapi/master/data/v2/csv/"
    }

    fn is_downloadable() -> bool {
        true
    }

    fn download_url() -> String {
        format!("{}{}.csv", Self::base_download_url(), Self::file_name())
    }

    fn load(base_path: &PathBuf) -> csv::Result<Vec<Self>>
    where
        Self: for<'de> serde::Deserialize<'de>,
    {
        let mut file_path = base_path.join(Self::file_name());
        file_path.set_extension("csv");
        let mut reader = Reader::from_path(file_path)?;
        reader.deserialize().collect()
    }

    fn get_metadata() -> CSVEntityMetaData {
        CSVEntityMetaData {
            file_name: Self::file_name().to_string(),
            download_url: Self::download_url(),
            is_downloadable: Self::is_downloadable(),
        }
    }
}

pub fn get_all_metadata() -> Vec<CSVEntityMetaData> {
    vec![
        AbilitiesCSV::get_metadata(),
        AbilityChangelogCSV::get_metadata(),
        AbilityChangelogProseCSV::get_metadata(),
        AbilityFlavorTextCSV::get_metadata(),
        AbilityNamesCSV::get_metadata(),
        AbilityEffectsCSV::get_metadata(),
        AbilityShortEffectsCSV::get_metadata(),
        BerriesCSV::get_metadata(),
        BerryFirmnessCSV::get_metadata(),
        BerryFirmnessNamesCSV::get_metadata(),
        BerryFlavorNamesCSV::get_metadata(),
        BerryFlavorPotenciesCSV::get_metadata(),
        EggGroupsCSV::get_metadata(),
        EggGroupProseCSV::get_metadata(),
        EvolutionChainsCSV::get_metadata(),
        EvolutionTriggersCSV::get_metadata(),
        EvolutionTriggerProseCSV::get_metadata(),
        GenerationNamesCSV::get_metadata(),
        GenerationsCSV::get_metadata(),
        GrowthRateProseCSV::get_metadata(),
        GrowthRatesCSV::get_metadata(),
        ItemCategoriesCSV::get_metadata(),
        ItemCategoryProseCSV::get_metadata(),
        ItemFlagsCSV::get_metadata(),
        ItemFlagDescriptionsCSV::get_metadata(),
        ItemFlagMapCSV::get_metadata(),
        ItemFlagNamesCSV::get_metadata(),
        ItemFlavorTextCSV::get_metadata(),
        ItemGameIndicesCSV::get_metadata(),
        ItemGenerationIdsCSV::get_metadata(),
        ItemNamesCSV::get_metadata(),
        ItemsCSV::get_metadata(),
        ItemEffectsCSV::get_metadata(),
        ItemShortEffectsCSV::get_metadata(),
        ItemPocketsCSV::get_metadata(),
        ItemPocketNamesCSV::get_metadata(),
        LanguagesCSV::get_metadata(),
        LanguageNamesCSV::get_metadata(),
        LocationAreasCSV::get_metadata(),
        LocationAreaEncounterRatesCSV::get_metadata(),
        LocationAreaProseCSV::get_metadata(),
        LocationsCSV::get_metadata(),
        LocationGameIndicesCSV::get_metadata(),
        LocationGenerationIdsCSV::get_metadata(),
        LocationNamesCSV::get_metadata(),
        LocationSubtitlesCSV::get_metadata(),
        MachinesCSV::get_metadata(),
        MovesCSV::get_metadata(),
        MoveChangelogCSV::get_metadata(),
        MoveDamageClassesCSV::get_metadata(),
        MoveDamageClassNamesCSV::get_metadata(),
        MoveDamageClassDescriptionsCSV::get_metadata(),
        MoveEffectsCSV::get_metadata(),
        MoveEffectChangelogCSV::get_metadata(),
        MoveEffectChangelogProseCSV::get_metadata(),
        MoveEffectEffectsCSV::get_metadata(),
        MoveEffectShortEffectsCSV::get_metadata(),
        MoveFlagsCSV::get_metadata(),
        MoveFlagDescriptionsCSV::get_metadata(),
        MoveFlagMapCSV::get_metadata(),
        MoveFlagNamesCSV::get_metadata(),
        MoveFlavorTextCSV::get_metadata(),
        MoveMetaCSV::get_metadata(),
        MoveMetaAilmentNamesCSV::get_metadata(),
        MoveMetaAilmentsCSV::get_metadata(),
        MoveMetaCategoriesCSV::get_metadata(),
        MoveMetaCategoryProseCSV::get_metadata(),
        MoveMetaStatChangesCSV::get_metadata(),
        MoveNamesCSV::get_metadata(),
        MoveTargetsCSV::get_metadata(),
        MoveTargetDescriptionsCSV::get_metadata(),
        MoveTargetNamesCSV::get_metadata(),
        PokemonCSV::get_metadata(),
        PokemonColorNamesCSV::get_metadata(),
        PokemonColorsCSV::get_metadata(),
        PokemonEvolutionCSV::get_metadata(),
        PokemonHabitatNamesCSV::get_metadata(),
        PokemonHabitatsCSV::get_metadata(),
        PokemonShapeAwesomeNamesCSV::get_metadata(),
        PokemonShapeDescriptionsCSV::get_metadata(),
        PokemonShapeNamesCSV::get_metadata(),
        PokemonShapesCSV::get_metadata(),
        PokemonSpeciesCSV::get_metadata(),
        PokemonSpeciesFlavorTextCSV::get_metadata(),
        PokemonSpeciesNamesCSV::get_metadata(),
        PokemonStatsCSV::get_metadata(),
        PokemonTypesCSV::get_metadata(),
        PokemonTypesPastCSV::get_metadata(),
        PokemonVersionIdsCSV::get_metadata(),
        RegionNamesCSV::get_metadata(),
        RegionsCSV::get_metadata(),
        StatNamesCSV::get_metadata(),
        StatsCSV::get_metadata(),
        TypeEfficacyCSV::get_metadata(),
        TypeEfficacyPastCSV::get_metadata(),
        TypeNamesCSV::get_metadata(),
        TypesCSV::get_metadata(),
        VersionGroupPokemonMoveMethodsCSV::get_metadata(),
        VersionGroupRegionsCSV::get_metadata(),
        VersionGroupsCSV::get_metadata(),
        VersionNamesCSV::get_metadata(),
        VersionsCSV::get_metadata(),
    ]
}

/// Generates a map where the keys are CSV file names and the values are the download URLs
pub fn get_download_map() -> HashMap<String, String> {
    get_all_metadata()
        .into_iter()
        .filter(|metadata| metadata.is_downloadable)
        .map(|metadata| (metadata.file_name, metadata.download_url))
        .collect()
}
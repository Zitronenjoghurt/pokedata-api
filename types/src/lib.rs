pub mod app_state;

pub mod entities {
    pub mod csv_entity;

    pub mod api {
        pub mod ability;
        pub mod generation;
        pub mod growth_rate;
        pub mod language;
        pub mod localized_values;
        pub mod pokemon;
        pub mod pokemon_color;
        pub mod pokemon_habitat;
        pub mod pokemon_shape;
        pub mod species;
        pub mod version;
        pub mod version_group;
    }

    pub mod csv {
        pub mod abilities;
        pub mod generation_names;
        pub mod generations;
        pub mod growth_rates;
        pub mod growth_rate_prose;
        pub mod languages;
        pub mod language_names;
        pub mod pokemon;
        pub mod pokemon_color_names;
        pub mod pokemon_colors;
        pub mod pokemon_habitat_names;
        pub mod pokemon_habitats;
        pub mod pokemon_shape_prose;
        pub mod pokemon_shapes;
        pub mod pokemon_species;
        pub mod pokemon_species_flavor_text;
        pub mod pokemon_species_names;
        pub mod version_group_pokemon_move_methods;
        pub mod version_group_regions;
        pub mod version_groups;
        pub mod version_names;
        pub mod versions;
    }

    pub mod traits {
        pub mod has_id;
        pub mod has_localized_values;
        pub mod has_version_id;
        pub mod id_value_pairing;
        pub mod into_id_map;
        pub mod into_localized_values_map;
        pub mod into_versioned_localized_values_map;
    }
}

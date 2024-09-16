pub mod app_state;

pub mod entities {
    pub mod csv_entity;

    pub mod api {
        pub mod ability;
        pub mod language;
        pub mod localized_values;
        pub mod pokemon_color;
        pub mod pokemon_shape;
        pub mod species;
    }

    pub mod csv {
        pub mod abilities;
        pub mod languages;
        pub mod language_names;
        pub mod pokemon_color_names;
        pub mod pokemon_colors;
        pub mod pokemon_shape_prose;
        pub mod pokemon_shapes;
        pub mod pokemon_species;
    }

    pub mod traits {
        pub mod has_id;
        pub mod has_localized_values;
        pub mod into_id_map;
        pub mod into_localized_values_map;
    }
}

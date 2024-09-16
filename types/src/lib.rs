pub mod app_state;

pub mod entities {
    pub mod csv_entity;

    pub mod api {
        pub mod ability;
        pub mod language;
        pub mod localized_names;
        pub mod species;
    }

    pub mod csv {
        pub mod abilities;
        pub mod languages;
        pub mod language_names;
        pub mod pokemon_species;
    }

    pub mod traits {
        pub mod has_id;
        pub mod has_localized_names;
        pub mod into_id_map;
        pub mod into_localized_names_map;
    }
}

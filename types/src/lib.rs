pub mod app_state;

pub mod entities {
    pub mod csv_entity;

    pub mod api {
        pub mod ability;
        pub mod species;
    }

    pub mod csv {
        pub mod abilities;
        pub mod pokemon_species;
    }

    pub mod traits {
        pub mod has_id;
        pub mod into_id_map;
    }
}

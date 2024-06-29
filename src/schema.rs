// @generated automatically by Diesel CLI.

diesel::table! {
    api_consumers (id) {
        id -> Int4,
        name -> Varchar,
        api_key -> Varchar,
    }
}

diesel::table! {
    consumersa (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    routes (id) {
        id -> Int4,
        path -> Varchar,
        private -> Bool,
        inner_path -> Varchar,
        upstream_id -> Int4,
    }
}

diesel::table! {
    targets (id) {
        id -> Int4,
        host -> Varchar,
        port -> Int4,
        upstream_id -> Int4,
    }
}

diesel::table! {
    upstreams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(routes -> upstreams (upstream_id));
diesel::joinable!(targets -> upstreams (upstream_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_consumers,
    consumersa,
    routes,
    targets,
    upstreams,
);

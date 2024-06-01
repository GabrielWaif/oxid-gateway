// @generated automatically by Diesel CLI.

diesel::table! {
    consumers (id) {
        id -> Int4,
        username -> Varchar,
        password -> Varchar,
    }
}

diesel::table! {
    routes (id) {
        id -> Int4,
        name -> Varchar,
        path -> Varchar,
        inner_path -> Varchar,
        upstream_id -> Int4,
    }
}

diesel::table! {
    targets (id) {
        id -> Int4,
        name -> Varchar,
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
    consumers,
    routes,
    targets,
    upstreams,
);

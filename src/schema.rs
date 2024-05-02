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
        upstream_id -> Nullable<Int4>,
    }
}

diesel::table! {
    target (id) {
        id -> Int4,
        name -> Varchar,
        host -> Varchar,
        port -> Int4,
    }
}

diesel::table! {
    target_upstream (upstream_id, target_id) {
        upstream_id -> Int4,
        target_id -> Int4,
    }
}

diesel::table! {
    upstreams (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::joinable!(routes -> upstreams (upstream_id));
diesel::joinable!(target_upstream -> target (target_id));
diesel::joinable!(target_upstream -> upstreams (upstream_id));

diesel::allow_tables_to_appear_in_same_query!(
    consumers,
    routes,
    target,
    target_upstream,
    upstreams,
);

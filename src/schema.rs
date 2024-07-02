// @generated automatically by Diesel CLI.

diesel::table! {
    api_consumers (id) {
        id -> Int4,
        name -> Varchar,
        api_key -> Varchar,
    }
}

diesel::table! {
    api_consumers_routes (api_consumer_id, route_id) {
        api_consumer_id -> Int4,
        route_id -> Int4,
    }
}

diesel::table! {
    routes (id) {
        id -> Int4,
        path -> Varchar,
        private -> Bool,
        inner_path -> Nullable<Varchar>,
        upstream_id -> Int4,
    }
}

diesel::table! {
    targets (id) {
        id -> Int4,
        protocol -> Varchar,
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

diesel::joinable!(api_consumers_routes -> api_consumers (api_consumer_id));
diesel::joinable!(api_consumers_routes -> routes (route_id));
diesel::joinable!(routes -> upstreams (upstream_id));
diesel::joinable!(targets -> upstreams (upstream_id));

diesel::allow_tables_to_appear_in_same_query!(
    api_consumers,
    api_consumers_routes,
    routes,
    targets,
    upstreams,
);

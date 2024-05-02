// @generated automatically by Diesel CLI.

diesel::table! {
    target (id) {
        id -> Int4,
        name -> Varchar,
        host -> Varchar,
        port -> Int4,
    }
}

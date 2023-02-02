// @generated automatically by Diesel CLI.

diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Varchar,
        completed -> Bool,
        completed_on -> Timestamp,
    }
}

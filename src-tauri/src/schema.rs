// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Text,
        name -> Text,
        date_created -> Timestamp,
        date_modified -> Timestamp,
    }
}

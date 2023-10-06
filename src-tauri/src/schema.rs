// @generated automatically by Diesel CLI.

diesel::table! {
    accounts (id) {
        id -> Text,
        name -> Text,
        date_created -> Timestamp,
        date_modified -> Timestamp,
    }
}

diesel::table! {
    books (id) {
        id -> Text,
        name -> Text,
        date_created -> Timestamp,
        date_modified -> Timestamp,
    }
}

diesel::table! {
    categories (id) {
        id -> Text,
    }
}

diesel::table! {
    statements (id) {
        id -> Integer,
        date_inserted -> Timestamp,
        hash -> Text,
        date_booked -> Timestamp,
        valuata -> Timestamp,
        bank -> Text,
        iban -> Nullable<Text>,
        reason -> Nullable<Text>,
        reference -> Nullable<Text>,
        balance_before -> Double,
        balance_after -> Double,
        account_id -> Text,
    }
}

diesel::joinable!(statements -> accounts (account_id));

diesel::allow_tables_to_appear_in_same_query!(
    accounts,
    books,
    categories,
    statements,
);

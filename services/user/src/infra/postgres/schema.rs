// @generated automatically by Diesel CLI.

diesel::table! {
    usr_main (id) {
        id -> Uuid,
        #[max_length = 255]
        email -> Varchar,
        password_token -> Text,
        verified -> Bool,
        person -> Nullable<Uuid>,
        date_created -> Timestamp,
        date_modified -> Timestamp,
        deleted -> Bool,
    }
}

diesel::table! {
    usr_metadata (id) {
        id -> Uuid,
        #[max_length = 255]
        key -> Varchar,
        value -> Nullable<Text>,
        user_id -> Uuid,
        date_created -> Timestamp,
        date_modified -> Timestamp,
    }
}

diesel::joinable!(usr_metadata -> usr_main (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    usr_main,
    usr_metadata,
);

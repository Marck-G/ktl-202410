// @generated automatically by Diesel CLI.

diesel::table! {
    prs_main (id) {
        id -> Uuid,
        customer_id -> Nullable<Uuid>,
        #[max_length = 100]
        given_name -> Varchar,
        #[max_length = 200]
        family_name -> Varchar,
        #[max_length = 200]
        additional_name -> Nullable<Varchar>,
        birth_date -> Nullable<Date>,
        #[max_length = 1]
        gender -> Nullable<Varchar>,
        date_created -> Nullable<Timestamp>,
        date_modified -> Nullable<Timestamp>,
    }
}

diesel::table! {
    prs_metadata (id) {
        id -> Uuid,
        person_id -> Uuid,
        #[max_length = 255]
        key -> Varchar,
        value -> Nullable<Text>,
        date_created -> Nullable<Timestamp>,
        date_modified -> Nullable<Timestamp>,
    }
}

diesel::joinable!(prs_metadata -> prs_main (person_id));

diesel::allow_tables_to_appear_in_same_query!(
    prs_main,
    prs_metadata,
);

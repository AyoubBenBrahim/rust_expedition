// @generated automatically by Diesel CLI.

diesel::table! {
    invitations (id) {
        id -> Uuid,
        #[max_length = 100]
        email -> Varchar,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (email) {
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 122]
        hash -> Varchar,
        created_at -> Timestamp,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    invitations,
    users,
);

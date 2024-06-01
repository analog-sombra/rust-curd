// @generated automatically by Diesel CLI.

pub mod sql_types {
    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "role"))]
    pub struct Role;

    #[derive(diesel::query_builder::QueryId, diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "status"))]
    pub struct Status;
}

diesel::table! {
    use diesel::sql_types::*;
    use super::sql_types::Role;
    use super::sql_types::Status;

    user (id) {
        id -> Int4,
        #[max_length = 255]
        user_id -> Varchar,
        #[max_length = 255]
        email -> Nullable<Varchar>,
        #[max_length = 255]
        username -> Nullable<Varchar>,
        #[max_length = 255]
        global_name -> Nullable<Varchar>,
        #[max_length = 255]
        avatar -> Nullable<Varchar>,
        mfa_enabled -> Bool,
        #[max_length = 255]
        banner -> Nullable<Varchar>,
        verified -> Bool,
        role -> Role,
        status -> Status,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

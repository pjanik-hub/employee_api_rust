// @generated automatically by Diesel CLI.

diesel::table! {
    employees (id) {
        id -> Uuid,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
    }
}

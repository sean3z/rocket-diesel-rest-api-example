// @generated automatically by Diesel CLI.

diesel::table! {
    heroes (id) {
        id -> Integer,
        #[max_length = 60]
        name -> Varchar,
        #[max_length = 60]
        identity -> Varchar,
        #[max_length = 60]
        hometown -> Varchar,
        age -> Integer,
    }
}

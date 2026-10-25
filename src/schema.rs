// `id` is nullable in this mapping so a single `Hero` can omit it on insert
// and let MySQL assign AUTO_INCREMENT. The SQL column itself is NOT NULL.
diesel::table! {
    heroes (id) {
        id -> Nullable<Integer>,
        #[max_length = 60]
        name -> Varchar,
        #[max_length = 60]
        identity -> Varchar,
        #[max_length = 60]
        hometown -> Varchar,
        age -> Integer,
    }
}

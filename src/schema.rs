table! {
    chapters (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
        published_at -> Nullable<Timestamp>,
        published -> Bool,
    }
}

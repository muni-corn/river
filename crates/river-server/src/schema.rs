table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        date_added -> Timestamptz,
        started -> Bool,
        percent_complete -> Nullable<Float8>,
        date_completed -> Nullable<Timestamptz>,
    }
}
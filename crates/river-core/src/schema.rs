table! {
    tasks (id) {
        id -> Int4,
        title -> Text,
        date_added -> Timestamptz,
        started -> Bool,
        percent_complete -> Nullable<Float4>,
        date_completed -> Nullable<Timestamptz>,
        creator -> Int4,
    }
}

table! {
    users (id) {
        id -> Int4,
        present -> Bool,
        current_task -> Nullable<Int4>,
        current_away_reason -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);

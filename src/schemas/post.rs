table! {
    post (t_id) {
        t_id -> Int4,
        title -> Varchar,
        message -> Text,
        views -> Int4,
        crated_on -> Timestamp,
        author_id -> Int4,
    }
}

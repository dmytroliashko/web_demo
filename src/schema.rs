table! {
    comments (id) {
        id -> Integer,
        author -> Varchar,
        content -> Text,
        createdat -> Datetime,
        post_id -> Integer,
    }
}

table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    comments,
    posts,
);

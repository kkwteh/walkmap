table! {
    maps (id) {
        id -> Text,
        user_id -> Nullable<Text>,
        created_at -> Int8,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(maps -> users (user_id));

allow_tables_to_appear_in_same_query!(maps, users,);

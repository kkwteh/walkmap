table! {
    maps (id) {
        id -> Text,
        user_id -> Nullable<Text>,
        created_at -> Int8,
    }
}

table! {
    markers (id) {
        id -> Text,
        map_id -> Text,
        order_parameter -> Float8,
        lat -> Float8,
        lon -> Float8,
        annotation -> Nullable<Text>,
        image_url -> Nullable<Text>,
    }
}

table! {
    users (id) {
        id -> Varchar,
        name -> Varchar,
    }
}

joinable!(maps -> users (user_id));
joinable!(markers -> maps (map_id));

allow_tables_to_appear_in_same_query!(
    maps,
    markers,
    users,
);

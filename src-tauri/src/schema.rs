table! {
    items (id) {
        id -> Integer,
        item_name -> Text,
        item_desc -> Text,
    }
}

table! {
    sales (id) {
        id -> Integer,
        item_id -> Integer,
        sale_value -> Integer,
        ts -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        password -> Text,
        full_name -> Text,
        active -> Bool,
        last_login -> Timestamp,
    }
}

joinable!(sales -> items (item_id));

allow_tables_to_appear_in_same_query!(
    items,
    sales,
    users,
);

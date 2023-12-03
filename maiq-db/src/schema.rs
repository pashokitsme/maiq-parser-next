// @generated automatically by Diesel CLI.

diesel::table! {
    groups (id) {
        id -> Integer,
        group_name -> Text,
    }
}

diesel::table! {
    target_groups (id) {
        id -> Integer,
        user_ref -> BigInt,
        group_name_ref -> Integer,
    }
}

diesel::table! {
    users (id) {
        id -> BigInt,
        cached_fullname -> Nullable<Text>,
        is_notifies_enabled -> Bool,
        is_broadcast_enabled -> Bool,
        modified_at -> Timestamp,
        created_at -> Timestamp,
    }
}

diesel::joinable!(target_groups -> groups (group_name_ref));

diesel::allow_tables_to_appear_in_same_query!(
    groups,
    target_groups,
    users,
);

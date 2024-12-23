// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        uuid -> Text,
        category_id -> Integer,
        title -> Text,
        description -> Text,
        completed -> Bool,
        images -> Text,
        tags -> Nullable<Text>,
        enable -> Bool,
        min_age -> Integer,
        url -> Text,
        content_type -> Text,
        width -> Integer,
        height -> Integer,
        bytes -> Integer,
        released_at -> Nullable<Timestamp>,
        broken_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    category (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    contents (id) {
        id -> Nullable<Integer>,
        episode_id -> Integer,
        uuid -> Text,
        index_no -> Integer,
        url -> Text,
        ads_url -> Nullable<Text>,
        content_type -> Text,
        width -> Integer,
        height -> Integer,
        bytes -> Integer,
        broken_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    episodes (id) {
        id -> Nullable<Integer>,
        album_id -> Integer,
        uuid -> Text,
        title -> Text,
        url -> Nullable<Text>,
        file_url -> Nullable<Text>,
        content_type -> Nullable<Text>,
        width -> Integer,
        height -> Integer,
        bytes -> Integer,
        broken_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(albums -> category (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    albums,
    category,
    contents,
    episodes,
);

// @generated automatically by Diesel CLI.

diesel::table! {
    albums (id) {
        id -> Integer,
        uuid -> Text,
        title -> Text,
        description -> Text,
        completed -> Bool,
        tags -> Nullable<Text>,
        enable -> Bool,
        min_age -> Integer,
        url -> Text,
        content_type -> Text,
        width -> Integer,
        height -> Integer,
        bytes -> Integer,
        released_at -> Nullable<Text>,
        broken_at -> Nullable<Text>,
        created_at -> Nullable<Text>,
        updated_at -> Nullable<Text>,
    }
}

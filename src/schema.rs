// @generated automatically by Diesel CLI.

diesel::table! {
    episodes (id) {
        id -> Nullable<Integer>,
        album_id -> Integer,
        title -> Nullable<Text>,
        uuid -> Nullable<Text>,
        url -> Nullable<Text>,
        broken_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
        updated_at -> Nullable<Timestamp>,
    }
}

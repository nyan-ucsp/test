// @generated automatically by Diesel CLI.

diesel::table! {
    episode (id) {
        id -> Nullable<Integer>,
        albumId -> Integer,
        title -> Nullable<Text>,
        uuid -> Nullable<Text>,
        url -> Nullable<Text>,
        brokenAt -> Nullable<Timestamp>,
        createdAt -> Nullable<Timestamp>,
        updatedAt -> Nullable<Timestamp>,
    }
}

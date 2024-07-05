// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Nullable<Integer>,
        text -> Text,
        added_at -> Timestamp,
        notify_at -> Nullable<Timestamp>,
    }
}

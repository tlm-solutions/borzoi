// @generated automatically by Diesel CLI.

diesel::table! {
    tetra_data (id) {
        id -> Int8,
        station -> Uuid,
        time -> Timestamptz,
        source_ssi -> Int4,
        destination_ssi -> Int4,
        protocol_identifier -> Int4,
        telegram_type -> Text,
        data -> Bytea,
        arbitrary -> Nullable<Json>,
    }
}

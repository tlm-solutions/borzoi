// @generated automatically by Diesel CLI.

diesel::table! {
    tetra_data (id) {
        id -> Int8,
        station -> Uuid,
        time -> Timestamptz,
        key -> Text,
        value -> Json,
    }
}

diesel::table! {
    tetra_failed_slots (id) {
        id -> Int8,
        station -> Uuid,
        time -> Timestamptz,
        burst_type -> Int4,
        slot_type -> Int4,
        first_slot_logical_channel -> Int4,
        first_slot_data -> Bytea,
        first_slot_crc_ok -> Bool,
        second_slot_present -> Bool,
        second_slot_logical_channel -> Nullable<Int4>,
        second_slot_data -> Nullable<Bytea>,
        second_slot_crc_ok -> Nullable<Bool>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    tetra_data,
    tetra_failed_slots,
);

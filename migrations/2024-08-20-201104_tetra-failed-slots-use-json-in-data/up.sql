-- Your SQL goes here

drop index tetra_failed_slots_time;

drop table tetra_failed_slots;

CREATE TABLE tetra_failed_slots(
    id BIGSERIAL PRIMARY KEY,
    station UUID NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    burst_type INT NOT NULL,
    slot_type INT NOT NULL,
    first_slot_logical_channel INT NOT NULL,
    first_slot_data JSON NOT NULL,
    first_slot_crc_ok BOOLEAN NOT NULL,
    second_slot_present BOOLEAN NOT NULL,
    second_slot_logical_channel INT CHECK (
        (second_slot_present = TRUE AND second_slot_logical_channel IS NOT NULL)
        OR (second_slot_present = FALSE AND second_slot_logical_channel IS NULL)
    ),
    second_slot_data JSON CHECK (
        (second_slot_present = TRUE AND second_slot_data IS NOT NULL)
        OR (second_slot_present = FALSE AND second_slot_data IS NULL)
    ),
    second_slot_crc_ok BOOLEAN CHECK (
        (second_slot_present = TRUE AND second_slot_crc_ok IS NOT NULL)
        OR (second_slot_present = FALSE AND second_slot_crc_ok IS NULL)
    )
);

create index tetra_failed_slots_time ON public.tetra_failed_slots using btree ( time );
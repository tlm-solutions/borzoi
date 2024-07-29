-- This file should undo anything in `up.sql`

drop index tetra_data_time;

drop table tetra_data;

CREATE TABLE tetra_data(
    id BIGSERIAL PRIMARY KEY,
    station UUID NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    source_ssi INT NOT NULL,
    destination_ssi INT NOT NULL,
    protocol_identifier INT NOT NULL,
    telegram_type TEXT NOT NULL,
    data BYTEA NOT NULL,
    arbitrary JSON);

create index tetra_data_time ON public.tetra_data using btree ( time );
-- Your SQL goes here
drop index tetra_data_time;

drop table tetra_data;

CREATE TABLE tetra_data(
    id BIGSERIAL PRIMARY KEY,
    station UUID NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    key TEXT NOT NULL,
    value JSON NOT NULL
);

create index tetra_data_time ON public.tetra_data using btree ( time );

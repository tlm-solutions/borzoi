-- Your SQL goes here

create index tetra_data_time ON public.tetra_data using btree ( time );
create index tetra_failed_slots_time ON public.tetra_failed_slots using btree ( time );

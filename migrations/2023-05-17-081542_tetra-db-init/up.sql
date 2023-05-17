-- Your SQL goes here

CREATE TABLE tetra_data(
    id BIGSERIAL PRIMARY KEY,
    station UUID NOT NULL,
    time TIMESTAMP WITH TIME ZONE NOT NULL,
    source_ssi INT NOT NULL,
    destination_ssi INT NOT NULL,
    protocol_identifier INT NOT NULL,
    telegram_type TEXT NOT NULL,
    data BYTEA NOT NULL
);

-- Your SQL goes here

CREATE TABLE banks (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    logo_path VARCHAR NOT NULL,
    name VARCHAR NOT NULL
);

SELECT
    diesel_manage_updated_at ('banks');

ALTER TABLE properties
ADD COLUMN bank_id INTEGER REFERENCES banks(id) ON UPDATE CASCADE ON DELETE CASCADE;

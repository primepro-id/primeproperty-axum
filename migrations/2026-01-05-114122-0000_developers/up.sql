-- Your SQL goes here

CREATE TABLE developers (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    logo_path VARCHAR NOT NULL,
    name VARCHAR NOT NULL
);

ALTER TABLE properties
ADD COLUMN developer_id INTEGER REFERENCES developers(id) ON UPDATE CASCADE ON DELETE CASCADE;

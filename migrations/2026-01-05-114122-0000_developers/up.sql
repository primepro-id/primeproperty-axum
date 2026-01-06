-- Your SQL goes here

CREATE TABLE developers (
    id SERIAL PRIMARY KEY,
    created_at TIMESTAMP NOT NULL DEFAULT NOW (),
    updated_at TIMESTAMP NOT NULL DEFAULT NOW (),
    picture_url VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    slug VARCHAR NOT NULL UNIQUE
);

ALTER TABLE properties
ADD COLUMN developer_slug VARCHAR REFERENCES developers(slug) ON UPDATE CASCADE;

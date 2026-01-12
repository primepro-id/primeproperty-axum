-- This file should undo anything in `up.sql`
ALTER TABLE properties
DROP COLUMN bank_id;

DROP TABLE banks;

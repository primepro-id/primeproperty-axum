-- This file should undo anything in `up.sql`
ALTER TABLE properties
DROP COLUMN developer_slug;

DROP TABLE developers;

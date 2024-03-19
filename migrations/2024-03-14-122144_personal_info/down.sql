-- This file should undo anything in `up.sql`
ALTER TABLE personal_info
    ALTER COLUMN birth_date TYPE varchar(255) USING birth_date::varchar(255);

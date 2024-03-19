-- Your SQL goes here
-- Your SQL goes here
-- Your SQL goes here
ALTER TABLE personal_info
    ALTER COLUMN birth_date TYPE date USING birth_date::date;

-- This file should undo anything in `up.sql`
DROP TRIGGER update_timestamp_onchange
ON chapters;
DROP FUNCTION update_timestamp_onchange;
DROP TABLE chapters;
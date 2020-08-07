-- Your SQL goes here

CREATE TABLE chapters
(
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  author VARCHAR NOT NULL,
  created_at TIMESTAMP DEFAULT current_timestamp,
  updated_at TIMESTAMP DEFAULT current_timestamp,
  published_at TIMESTAMP NULL DEFAULT NULL,
  published BOOLEAN NOT NULL DEFAULT 'f'
);

CREATE FUNCTION update_timestamp_onchange()
RETURNS TRIGGER AS $$
BEGIN
RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_timestamp_onchange BEFORE
UPDATE
    ON chapters FOR EACH ROW
EXECUTE PROCEDURE 
    update_timestamp_onchange()
-- Add migration script here
CREATE TABLE IF NOT EXISTS ShortenedURL  (
  original_url varchar(255) NOT NULL,
  short_code varchar(32) NOT NULL UNIQUE
);

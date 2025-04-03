-- Add down migration script here
DROP INDEX users_email_idx;
DROP INDEX users_username_idx;
DROP TABLE users;
DROP EXTENSION "uuid-ossp";
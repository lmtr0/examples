-- CREATE DATABASE testdb;
CREATE SCHEMA if not exists testdb;
create table testdb.books (
    id uuid not null,
    title text not null,
    publish_date timestamptz not null,
    meta jsonb not null,
    PRIMARY KEY (id)
);

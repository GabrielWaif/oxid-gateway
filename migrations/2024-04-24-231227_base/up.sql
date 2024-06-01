-- Your SQL goes here
CREATE TABLE upstreams (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE targets (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  host VARCHAR NOT NULL,
  port INTEGER NOT NULL,
  upstream_id INTEGER NOT NULL REFERENCES upstreams(id)
);

CREATE TABLE routes (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  path VARCHAR NOT NULL,
  inner_path VARCHAR NOT NULL,
  upstream_id INTEGER NOT NULL REFERENCES upstreams(id)
);

CREATE TABLE consumers (
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL
)

-- Your SQL goes here
CREATE TABLE upstreams (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE targets (
  id SERIAL PRIMARY KEY,
  host VARCHAR NOT NULL,
  port INTEGER NOT NULL,
  upstream_id INTEGER NOT NULL REFERENCES upstreams(id)
);

CREATE TABLE routes (
  id SERIAL PRIMARY KEY,
  path VARCHAR NOT NULL,
  private BOOLEAN NOT NULL,
  inner_path VARCHAR NOT NULL,
  upstream_id INTEGER NOT NULL REFERENCES upstreams(id)
);

CREATE TABLE api_consumers (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  api_key VARCHAR NOT NULL
)

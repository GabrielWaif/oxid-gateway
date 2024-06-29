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
  path VARCHAR UNIQUE NOT NULL,
  private BOOLEAN NOT NULL,
  inner_path VARCHAR,
  upstream_id INTEGER NOT NULL REFERENCES upstreams(id)
);

CREATE TABLE api_consumers (
  id SERIAL PRIMARY KEY,
  name VARCHAR UNIQUE NOT NULL,
  api_key VARCHAR UNIQUE NOT NULL
)

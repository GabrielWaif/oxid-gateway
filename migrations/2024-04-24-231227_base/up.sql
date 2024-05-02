-- Your SQL goes here
CREATE TABLE target (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  host VARCHAR NOT NULL,
  port INTEGER NOT NULL
);

CREATE TABLE upstreams (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL
);

CREATE TABLE target_upstream (
  upstream_id INTEGER REFERENCES upstreams(id),
  target_id INTEGER REFERENCES target(id),
  PRIMARY KEY(upstream_id, target_id)
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

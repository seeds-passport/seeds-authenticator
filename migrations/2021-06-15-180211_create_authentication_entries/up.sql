CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE authentication_entries (
  id UUID DEFAULT uuid_generate_v4 (),
  account_name VARCHAR NOT NULL, 
  secret UUID NOT NULL,
  policy JSONB NOT NULL,
  policy_base64 TEXT NOT NULL,
  valid_until TIMESTAMP NOT NULL,
  blockchain_index BIGINT DEFAULT NULL,
  PRIMARY KEY (id)
)
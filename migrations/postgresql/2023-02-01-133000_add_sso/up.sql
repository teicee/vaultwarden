CREATE TABLE sso_nonce (
  nonce    CHAR(36) NOT NULL PRIMARY KEY
);

CREATE TABLE sso_settings (
  id            INTEGER NOT NULL PRIMARY KEY,
  enabled       BOOLEAN NOT NULL,
  force         BOOLEAN NOT NULL,
  client_id     TEXT NOT NULL,
  client_secret TEXT NOT NULL,
  authority     TEXT NOT NULL
);

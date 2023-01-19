CREATE TABLE sso_nonce (
  uuid     CHAR(36) NOT NULL PRIMARY KEY,
  org_uuid CHAR(36) NOT NULL REFERENCES organizations (uuid),
  nonce    CHAR(36) NOT NULL
);
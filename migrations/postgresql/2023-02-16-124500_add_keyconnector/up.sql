ALTER TABLE users
ADD COLUMN uses_key_connector BOOLEAN;

CREATE TABLE keyconnector (
  uuid      CHAR(36) NOT NULL PRIMARY KEY,
  user_uuid CHAR(36) NOT NULL REFERENCES users (uuid),
  key      TEXT     NOT NULL,
  UNIQUE (user_uuid, type)
);

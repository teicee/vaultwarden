use crate::api::EmptyResult;
use crate::db::DbConn;
use crate::error::MapResult;
use crate::CONFIG;

use serde_json::Value;

db_object! {
    #[derive(Identifiable, Queryable, Insertable)]
    #[diesel(table_name = sso_nonce)]
    #[diesel(primary_key(nonce))]
    pub struct SsoNonce {
        pub nonce: String,
    }

    #[derive(Identifiable, Queryable, Insertable, AsChangeset, Deserialize, Serialize)]
    #[diesel(table_name = sso_settings)]
    #[diesel(primary_key(id))]
    pub struct SsoSettings {
        id: i32,
        pub enabled: bool,
        pub force: bool,
        pub client_id: String,
        pub client_secret: String,
        pub authority: String,
    }
}

/// Local methods
impl SsoNonce {
    pub fn new(nonce: String) -> Self {
        Self {
            nonce,
        }
    }
}

/// Database methods
impl SsoNonce {
    pub async fn save(&self, conn: &mut DbConn) -> EmptyResult {
        db_run! { conn:
            sqlite, mysql {
                diesel::replace_into(sso_nonce::table)
                    .values(SsoNonceDb::to_db(self))
                    .execute(conn)
                    .map_res("Error saving SSO device")
            }
            postgresql {
                let value = SsoNonceDb::to_db(self);
                diesel::insert_into(sso_nonce::table)
                    .values(&value)
                    .execute(conn)
                    .map_res("Error saving SSO nonce")
            }
        }
    }

    pub async fn delete(self, conn: &mut DbConn) -> EmptyResult {
        db_run! { conn: {
            diesel::delete(sso_nonce::table.filter(sso_nonce::nonce.eq(self.nonce)))
                .execute(conn)
                .map_res("Error deleting SSO nonce")
        }}
    }

    pub async fn find(nonce: &str, conn: &mut DbConn) -> Option<Self> {
        db_run! { conn: {
            sso_nonce::table
                .filter(sso_nonce::nonce.eq(nonce))
                .first::<SsoNonceDb>(conn)
                .ok()
                .from_db()
        }}
    }
}

/// Local methods
impl SsoSettings {
    pub fn new() -> Self {
        Self {
            id: 0,
            enabled: false,
            force: false,
            client_id: String::new(),
            client_secret: String::new(),
            authority: String::new(),
        }
    }

    pub fn from_data(enabled: bool, force: bool, client_id: String, client_secret: String, authority: String) -> Self {
        Self {
            id: 0,
            enabled: enabled,
            force: force,
            client_id: client_id,
            client_secret: client_secret,
            authority: authority,
        }
    }
}

/// Database methods
impl SsoSettings {
    pub async fn save(&self, conn: &mut DbConn) -> EmptyResult {
        db_run! { conn:
            sqlite, mysql {
                diesel::replace_into(sso_settings::table)
                    .values(SsoSettingsDb::to_db(self))
                    .execute(conn)
                    .map_res("Error saving SSO settings")
            }
            postgresql {
                let value = SsoSettingsDb::to_db(self);
                diesel::insert_into(sso_settings::table)
                    .values(&value)
                    .execute(conn)
                    .map_res("Error saving SSO settings")
            }
        }
    }

    pub async fn get(conn: &mut DbConn) -> Option<Self> {
        db_run! { conn: {
            sso_settings::table
                .first::<SsoSettingsDb>(conn).ok().from_db()
        }}
    }

    pub fn to_json(&self) -> Value {
        json!({
            "enabled": self.enabled,
            "force": self.force,
            "client_id": self.client_id,
            "client_secret": self.client_secret,
            "authority": self.authority,
            "callback_path": self.generate_sso_callback_path(),
        })
    }

    pub fn generate_sso_callback_path(&self) -> String {
        let domain = CONFIG.domain();
        format!("{domain}/identity/connect/oidc-signin")
    }
}
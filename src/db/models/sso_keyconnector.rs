use crate::api::EmptyResult;
use crate::db::DbConn;
use crate::error::MapResult;
use super::User;

db_object! {
    #[derive(Identifiable, Queryable, Insertable, Associations, AsChangeset)]
    #[diesel(table_name = keyconnector)]
    #[diesel(belongs_to(User, foreign_key = user_uuid))]
    #[diesel(primary_key(uuid))]
    pub struct SsoKeyConnector {
        pub uuid: String,
        pub user_uuid: String,
        pub secretkey: String,
    }
}

/// Local methods
impl SsoKeyConnector {
    pub fn new(user_uuid: String) -> Self {
        Self {
            uuid: crate::util::get_uuid(),
            user_uuid,
            secretkey: String::new(),
        }
    }  
}

/// Database methods
impl SsoKeyConnector {
    pub async fn save(&self, conn: &DbConn) -> EmptyResult {
        db_run! { conn:
            sqlite, mysql {
                match diesel::replace_into(keyconnector::table)
                    .values(SsoKeyConnectorDb::to_db(self))
                    .execute(conn)
                {
                    Ok(_) => Ok(()),
                    // Record already exists and causes a Foreign Key Violation because replace_into() wants to delete the record first.
                    Err(diesel::result::Error::DatabaseError(diesel::result::DatabaseErrorKind::ForeignKeyViolation, _)) => {
                        diesel::update(keyconnector::table)
                            .filter(keyconnector::uuid.eq(&self.uuid))
                            .set(SsoKeyConnectorDb::to_db(self))
                            .execute(conn)
                            .map_res("Error adding keyconnector config to organization")
                    }
                    Err(e) => Err(e.into()),
                }.map_res("Error adding keyconnector config to organization")
            }
            postgresql {
                let value = SsoKeyConnectorDb::to_db(self);
                diesel::insert_into(keyconnector::table)
                    .values(&value)
                    .on_conflict(keyconnector::uuid)
                    .do_update()
                    .set(&value)
                    .execute(conn)
                    .map_res("Error adding keyconnector config to organization")
            }
        }
    }

    pub async fn delete(self, conn: &DbConn) -> EmptyResult {
        db_run! { conn: {
            diesel::delete(keyconnector::table.filter(keyconnector::uuid.eq(self.uuid)))
                .execute(conn)
                .map_res("Error deleting SSO Config")
        }}
    }

    pub async fn find_by_userid(user_uuid: &str, conn: &DbConn) -> Option<Self> {
        db_run! { conn: {
            keyconnector::table
                .filter(keyconnector::user_uuid.eq(user_uuid))
                .first::<SsoKeyConnectorDb>(conn)
                .ok()
                .from_db()
        }}
    }
}
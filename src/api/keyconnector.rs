use crate::{
    api::{EmptyResult, JsonResult, JsonUpcase},
    auth::Headers,
    db::{models::*, DbConn},
};
use rocket::serde::json::Json;
use rocket::Route;

pub fn routes() -> Vec<Route> {
    routes![get_userkeys, post_userkeys]
}

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
pub struct KeyConnectorData {
    Key: String,
}

#[get("/user-keys")]
async fn get_userkeys(headers: Headers, conn: DbConn) -> JsonResult {
    let user = headers.user;
    match SsoKeyConnector::find_by_userid(&user.uuid, &conn).await {
        Some(keyconnector_key) => Ok(Json(json!({
           "Key": keyconnector_key.secretkey,
        }))),
        None => Ok(Json(json!({
           "Key":null,
        }))),
    }
}

#[post("/user-keys", data = "<data>")]
async fn post_userkeys(data: JsonUpcase<KeyConnectorData>, headers: Headers, conn: DbConn) -> EmptyResult {
    let user = headers.user;
    let user_uuid = user.uuid;
    let data: KeyConnectorData = data.into_inner().data;

    let mut keyconnector = match SsoKeyConnector::find_by_userid(&user_uuid, &conn).await {
        Some(keyconnector) => keyconnector,
        None => SsoKeyConnector::new(user_uuid),
    };

    keyconnector.secretkey = data.Key;
    keyconnector.save(&conn).await?;

    Ok(())
}

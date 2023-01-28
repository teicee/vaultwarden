use rocket::serde::json::Json;
use rocket::{
    http::Status,
    Route,
};
use crate::{
    api::JsonResult,
    auth::Headers,
    crypto,
};


pub fn routes() -> Vec<Route> {
    routes![get_userkeys, put_userkeys, post_userkeys]
}

#[get("/user-keys")]
fn get_userkeys(headers: Headers) -> JsonResult {
    let user = headers.user;
    if !user.akey.is_empty() {
        Ok(Json(json!({
            "Key": crypto::encodebase64(user.akey),
         })))
    } else {
        err_code!("User doesn't exist", Status::NotFound.code);
    }
}

#[put("/user-keys")]
fn put_userkeys(headers: Headers) -> JsonResult {
    let user = headers.user;
    Ok(Json(json!({
        "Key": user.akey,
    })))
}

#[post("/user-keys")]
fn post_userkeys(headers: Headers) -> JsonResult {
    let user = headers.user;
    Ok(Json(json!({
        "Key": user.akey,
    })))
}
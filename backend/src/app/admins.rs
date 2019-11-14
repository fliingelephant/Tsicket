use std::env;

use actix_web::{Error, HttpRequest, HttpResponse, web::Query};
use futures::{Future, future::result};
use md5::compute;
use serde::Deserialize;

use crate::app::ADMIN_ID;
use crate::app::ADMIN_PASSWORD_WITH_SALT;

use crate::db::events;

#[inline]
fn md5_with_salt(id: &String, raw_password: &String) -> String {
    format!("{:x}", md5::compute(raw_password.to_owned() + id))
}

#[derive(Debug, Deserialize)]
pub struct LoginAdmin {
    pub admin_id: String,
    pub password: String,
}

#[inline]
pub fn login(
    Query(login_admin): Query<LoginAdmin>
) -> impl Future<Item=HttpResponse, Error=Error> {
    let password = md5_with_salt(&login_admin.admin_id, &login_admin.password);

    println!("{}",login_admin.admin_id);
    println!("{}",password);
    if (login_admin.admin_id == *ADMIN_ID) && (password == *ADMIN_PASSWORD_WITH_SALT) {
        // TODO: cookie
        result(Ok(HttpResponse::Ok().json("Success.")))
    } else {
        // TODO: rejection
        result(Ok(HttpResponse::Ok().json("Wrong!")))
    }
}

#[inline]
pub fn logout(
    Query(login_admin): Query<LoginAdmin>
) -> impl Future<Item=HttpResponse, Error=Error> {
    // TODO
    result(Ok(HttpResponse::Ok().json("Hello World!")))
}

pub fn get_all_events(
    _req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(Ok(HttpResponse::Ok().json("Hello World!")))

}
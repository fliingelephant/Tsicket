use std::convert::From;

use actix_web::{Error, HttpRequest, HttpResponse, ResponseError, web::Data, web::Json, web::Query};
use futures::{Future, future::result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::utils::auth;

lazy_static! {
    static ref RE_USERNAME: Regex = Regex::new(r"^[_0-9a-zA-Z]+$").unwrap();
}

#[derive(Debug, Deserialize)]
pub struct In<U> {
    user: U,
}

#[derive(Debug, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterUser {
    #[validate(
    length(
    min = "1",
    max = "20",
    message = "fails validation - must be 1-20 characters long"
    ),
    )]
    pub username: String,
    #[validate(length(
    min = "8",
    max = "72",
    message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
}

#[derive(Debug, Validate, Deserialize)]
pub struct LoginUser {
    #[validate(
    length(
    min = "1",
    max = "20",
    message = "fails validation - must be 1-20 characters long"
    )
    )]
    pub username: String,
    #[validate(length(
    min = "8",
    max = "72",
    message = "fails validation - must be 8-72 characters long"
    ))]
    pub password: String,
    #[validate(range(
    min = "0",
    max = "2",
    message = "fails validation - must be one of {0, 1, 2}"
    ))]
    pub usertype: u8,
}

pub fn register(
    /*
    form: Json<In<RegisterUser>>,
) -> impl Future<Item = HttpResponse, Error = Error> {*/
    Query(register_user): Query<RegisterUser>
) -> impl Future<Item=HttpResponse, Error=Error> {

    result(Ok(HttpResponse::Ok().json("Hello World!")))
    /*
    match register_user.validate() {
        Ok(_) => format!("register request for client with username={} and password={}",
                         register_user.username, register_user.password),
        Err(e) => format!("{}", e),
    }*/

    /*
    result(register_user.validate())
        .from_err()
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json(res)),
            Err(e) => Ok(e.error_response()),
        })
     */
}

pub fn login(
    Query(login_user): Query<LoginUser>
) -> impl Future<Item=HttpResponse, Error=Error> {

    result(Ok(HttpResponse::Ok().json("Hello World!")))
    /*
match login_user.validate() {
    Ok(_) => format!("login request for client with username={} and password={}",
                     login_user.username, login_user.password),
    Err(e) => format!("{}", e),
}*/
    /*
    let login_user = form.into_inner().user;

    Ok(HttpResponse::Ok().finish())
    */
}
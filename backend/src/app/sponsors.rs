use std::convert::From;

use actix_identity::{Identity};
use actix_web::{Error,
    HttpRequest,
    HttpResponse,
    ResponseError,
    web::Data,
    web::Form,
    web::Json};
use futures::{Future, future::result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::events;
use crate::db::sponsors;

/*
use super::AppState;
use crate::models::User;
*/

lazy_static! {
    static ref RE_SPONSORNAME: Regex = Regex::new(r"^[_0-9a-zA-Z\u4e00-\u9f5]+$").unwrap();
}

// TODO: trait
#[derive(Serialize)]
pub struct TestResponse {
    pub content: Vec<events::Event>,
    pub msg: String,
    pub status: u8,
}

#[derive(Debug, Validate, Deserialize)]
pub struct RegisterSponsor {
    pub sponsorname: String,
    pub password: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginSponsor {
    pub id: String,
    pub password: String,
}

#[derive(Debug)]
pub struct QuerySponsor {
    pub sponsor_name: String,
}

pub fn register(
    id: Identity,
    register_sponsor: Json<RegisterSponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    /*
    result(register_sponsor.validate())
        .from_err()
        .and_then(move |_| sponsors::sponsor_register(
            &register_sponsor.id,
            &register_sponsor.sponsorname,
            &register_sponsor.password))
        .and_then(|res| match res {
            Ok(res) => Ok(HttpResponse::Ok().json("Hello World!")),
            Err(e) => Err(e),
            _ => {}
        })*/
    //result(Ok(HttpResponse::Ok().json("Hello World!")))

    result(match sponsors::sponsor_register(
        &register_sponsor.id,
        &register_sponsor.sponsorname,
        &register_sponsor.password) {
        Ok(()) => {
            id.remember(register_sponsor.id.to_owned());
            Ok(HttpResponse::Ok().finish())
        },
        Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)),
    })
}

pub fn login(
    id: Identity,
    login_sponsor: Json<LoginSponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match sponsors::sponsor_log_in(
        &login_sponsor.id,
        &login_sponsor.password) {
        Ok(()) => {
            id.remember(login_sponsor.id.to_owned());
            Ok(HttpResponse::Ok().finish())
        },
        Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)),
    })
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

#[inline]
pub fn logout(
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    id.forget();
    result(Ok(HttpResponse::Ok().finish()))
}

pub fn get_events(
    sponsor: Json<QuerySponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    let mut event_list: Vec<events::Event> = vec![];
    let t = sponsors::get_sponsor_events(
        &sponsor.sponsor_name, &mut event_list);
    result(match t {
        Ok(()) => Ok(HttpResponse::Ok().json(TestResponse {
            content: event_list,
            msg: "".to_string(),
            status: 0,
        })),
        Err(e) => Ok(HttpResponse::Ok().json(TestResponse {
            content: event_list,
            msg: e,
            status: 1,
        }))
    })
}

use std::convert::From;

use actix_web::{Error, HttpRequest, HttpResponse, ResponseError, web::Data, web::Json, web::Query};
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
    #[validate(
    length(
    min = "1",
    max = "20",
    message = "账户名不能少于一个字符或者超过二十个字符。"
    ),
    regex(
    path = "RE_SPONSORNAME",
    message = "账户名必须由汉字、英文字母、数字或下划线构成。"
    )
    )]
    pub sponsorname: String,
    #[validate(
    length(
    min = "8",
    max = "72",
    message = "密码不能少于七个字符或者超过七十二个字符。"
    ))]
    pub password: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginSponsor {
    pub account_id: String,
    pub password: String,
}

#[derive(Debug)]
pub struct QuerySponsor {
    pub sponsor_name: String,
}

pub fn register(
    Query(register_sponsor): Query<RegisterSponsor>,
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
        Ok(()) => Ok(HttpResponse::Ok().json("Hello World!")),
        Err(e) => Ok(HttpResponse::Ok().json(e)),
    })
}

pub fn login(
    Query(login_user): Query<LoginSponsor>,
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

#[inline]
pub fn logout(
    Query(login_sponsor): Query<LoginSponsor>
) -> impl Future<Item=HttpResponse, Error=Error> {
    // TODO
    result(Ok(HttpResponse::Ok().json("Not implemented api.")))
}

pub fn get_events(
    Query(sponsor): Query<QuerySponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    let mut event_list: Vec<events::Event> = vec![];
    let t = events::get_sponsor_events(
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
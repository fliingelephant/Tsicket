use std::convert::From;
use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{Error,
    HttpRequest,
    HttpResponse,
    ResponseError,
    web::Data,
    web::Form,
    web::Json,
    web::Query};
use futures::{Future, future::result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::db::events;
use crate::db::events::Event;
use crate::db::sponsors;

use super::EventState;

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

#[derive(Debug, Deserialize)]
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

#[derive(Deserialize)]
pub struct QuerySponsor {
    pub sponsor_name: String,
}

pub fn register(
    id: Identity,
    register_sponsor: Json<RegisterSponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
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
        Ok(name) => {
            id.remember(login_sponsor.id.to_owned());
            Ok(HttpResponse::Ok().json(name))
        },
        Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)),
    })
}

#[inline]
pub fn logout(
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    id.forget();
    result(Ok(HttpResponse::Ok().finish()))
}

pub fn publish_event(
    (event, id, state):
        (Json<Event>, Identity, Data<Mutex<EventState>>),
) -> impl Future<Item=HttpResponse, Error=Error> {
    if let Some(id) = id.identity() {
        let mut state = state.lock().unwrap();
        let new_event = event.into_inner();
        state.event_list.insert(new_event.event_name.clone(), new_event);
        result(Ok(HttpResponse::Ok().finish()))
    } else {
        result(Ok(HttpResponse::NonAuthoritativeInformation().finish()))
    }
}

pub fn get_events(
    (Query(sponsor), id, state):
        (Query<QuerySponsor>, Identity, Data<Mutex<EventState>>),
) -> impl Future<Item=HttpResponse, Error=Error> {
    let mut sponsor_event_list: Vec<Event> = vec![];
    let t = events::get_sponsor_events(
        &sponsor.sponsor_name, &mut sponsor_event_list);
    result(match t {
        Ok(()) => 
            if (sponsor_event_list.len() == 0) {
                Ok(HttpResponse::Ok().json(TestResponse {
                    content: sponsor_event_list,
                    msg: "".to_string(),
                    status: 0,
            }))} else {
                Ok(HttpResponse::Ok().json(TestResponse {
                    content: sponsor_event_list,
                    msg: "".to_string(),
                    status: 1,
            }))},
        Err(e) => Ok(HttpResponse::Ok().
        json(TestResponse {
            content: sponsor_event_list,
            msg: e,
            status: 2,
        })
        )
    })
}

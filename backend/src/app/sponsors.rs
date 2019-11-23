use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{Error,
    HttpResponse,
    web::Data,
    web::Json};
use futures::{Future, future::result};
use serde::{Deserialize, Serialize};

use crate::db::events;
use crate::db::events::Event;
use crate::db::sponsors;

use super::EventState;


#[derive(Debug, Deserialize)]
pub struct RegisterSponsor {
    pub sponsorname: String,
    pub password: String,
    pub id: String,
    pub email: String,
    pub phone_number: String,
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

pub fn register(
    id: Identity,
    register_sponsor: Json<RegisterSponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match sponsors::sponsor_register(
        &register_sponsor.id,
        &register_sponsor.sponsorname,
        &register_sponsor.password,
        &register_sponsor.email,
        &register_sponsor.phone_number) {
        Ok(()) => {
            id.remember(register_sponsor.sponsorname.to_owned());
            Ok(HttpResponse::Ok().finish())
        },
        Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)),
    })
}

#[inline]
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
        result(Ok(HttpResponse::Unauthorized().finish()))
    }
}

#[derive(Serialize)]
pub struct EventRet {
    events: Vec<Event>
}

#[inline]
pub fn get_events(
    (id, state):
        (Identity, Data<Mutex<EventState>>),
) -> impl Future<Item=HttpResponse, Error=Error> {
    if let Some(sponsor_name) = id.identity() {
        let mut sponsor_event_list: Vec<Event> = vec![];
        /*
        let t = events::get_sponsor_events(
            &sponsor_id, &mut sponsor_event_list);
        result(match t {
            Ok(()) => Ok(HttpResponse::Ok().json(
                    EventRet {
                        events: sponsor_event_list
                    }
                )),
            Err(e) => Ok(HttpResponse::ServiceUnavailable().json(e))
        })*/
        let mut state = state.lock().unwrap();
        for (name, event) in &state.event_list {
            if (event.sponsor_name == sponsor_name) {
                sponsor_event_list.push(event.clone())
            }
        }
        result(Ok(HttpResponse::Ok().json(EventRet {
                events: sponsor_event_list
            }
        )))
    } else {
        result(Ok(HttpResponse::Unauthorized().finish()))
    }
}

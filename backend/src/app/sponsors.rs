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
use super::events::EventsRet;
use super::EVENT_LIST;


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
            id.remember(name.to_owned());
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

pub fn publish_event(
    (event, id):
        (Json<Event>, Identity,
            // Data<Mutex<EventState>>
        ),
) -> impl Future<Item=HttpResponse, Error=Error> {
    if let Some(id) = id.identity() {
        //let mut state = state.lock().unwrap();
        let new_event = event.into_inner();
        //state.event_list.push(new_event.clone());
        EVENT_LIST.lock().unwrap().push(new_event.clone());
        result(Ok(HttpResponse::Ok().finish()))
    } else {
        result(Ok(HttpResponse::Unauthorized().finish()))
    }
}

pub fn get_events(
    (id):
        (Identity),
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

        //let mut state = state.lock().unwrap();
        for event in &(*EVENT_LIST.lock().unwrap()) {
            println!("{:?}",event);
            if (event.sponsor_name == sponsor_name) {
                sponsor_event_list.push(event.clone())
            }
        }
        result(Ok(HttpResponse::Ok().json(EventsRet {
                events: sponsor_event_list
            }
        )))
    } else {
        result(Ok(HttpResponse::Unauthorized().finish()))
    }
}

pub fn alter_sponsor_info(
    (alter_sponsor, id):
        (Json<sponsors::Sponsor>, Identity)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(_) => {
            let alter_sponsor = alter_sponsor.into_inner();
            match sponsors::alter_sponsor_info(&alter_sponsor) {
                Ok(()) => Ok(HttpResponse::Ok().finish()), // 200 Ok
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

pub fn get_sponsor_info (
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(name) => {
            match sponsors::get_info_by_name(&name) {
                Ok(sponsor) => Ok(HttpResponse::Ok().json(&sponsor)), // 200 Ok
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
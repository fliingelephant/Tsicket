use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{Error,
    HttpResponse,
    web::Data,
    web::Json};
use futures::{Future, future::result};
use serde::{Deserialize, Serialize};

use crate::db::events::{Event};
use crate::db::sponsors;
use super::events::EventsRet;
use super::EVENT_LIST;
use super::update::update_events;
use crate::utils::auth::{identify_sponsor};


#[derive(Deserialize)]
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

#[derive(Deserialize)]
pub struct QuerySponsorByID {
    pub sponsor_id: String,
}

#[allow(dead_code)]
#[inline]
pub fn login(
    id: Identity,
    login_sponsor: Json<LoginSponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match sponsors::sponsor_log_in(
        &login_sponsor.id,
        &login_sponsor.password) {
        Ok(name) => {
            id.remember("1".to_owned() + &name);
            Ok(HttpResponse::Ok().json(name)) // 200 OK
        },
        Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)), // 422 Unprocessable Entity
    })
}

#[allow(dead_code)]
#[inline]
pub fn logout(
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(_) => {
            id.forget();
            Ok(HttpResponse::Ok().finish()) // 200 OK
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
#[inline]
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
            id.remember("1".to_owned() + &register_sponsor.sponsorname);
            Ok(HttpResponse::Ok().finish())
        },
        Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)),
    })
}

#[allow(dead_code)]
#[inline]
pub fn publish_event(
    event: Json<Event>,
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(_) => {
            let new_event = event.into_inner();
            EVENT_LIST.lock().unwrap()
                .entry(new_event.event_id.clone())
                .or_insert(new_event.clone());
            //update_events(&events);

            Ok(HttpResponse::Ok().finish()) // 200 OK
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
#[inline]
pub fn get_available_events(
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            let mut sponsor_event_list: Vec<Event> = vec![];

            for (_, event) in &*EVENT_LIST.lock().unwrap() {
                if event.sponsor_name == sponsor_name {
                    sponsor_event_list.push(event.clone())
                }
            }
            Ok(HttpResponse::Ok().json(EventsRet {
                    events: sponsor_event_list
                }
            ))
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
#[inline]
pub fn alter_sponsor_info(
    alter_sponsor: Json<sponsors::Sponsor>,
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            let alter_sponsor = alter_sponsor.into_inner();
            if sponsor_name != alter_sponsor.sponsor_name {
                Ok(HttpResponse::Unauthorized().finish())
            } else {
                match sponsors::alter_sponsor_info(&alter_sponsor) {
                    Ok(()) => Ok(HttpResponse::Ok().finish()), // 200 Ok
                    Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
                }
            }
        }
        Err(()) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
#[inline]
pub fn get_sponsor_info (
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            match sponsors::get_info_by_name(&sponsor_name) {
                Ok(sponsor) => Ok(HttpResponse::Ok().json(&sponsor)), // 200 Ok
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(()) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
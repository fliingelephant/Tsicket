use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{Error,
    HttpResponse,
    web::Data,
    web::Json};
use futures::{Future, future::result};
use md5::compute;
use rand::{thread_rng, Rng};
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
            println!("sponsor {} log in", name);
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

#[derive(Deserialize)]
pub struct PublishEvent {
    pub event_name: String,
    pub start_time: String,
    pub end_time: String,
    pub event_type: i8,
    pub event_introduction: String,
    pub event_picture: String,
    pub event_capacity: i32,
    pub left_tickets: i32,
    pub event_location: String,
}

#[inline]
fn md5(x: &String) -> String {
    format!("{:x}", compute(x.to_owned()))
}

#[derive(Serialize)]
pub struct EventIDRet {
    event_id: String
}

#[allow(dead_code)]
pub fn publish_event(
    id: Identity,
    new_event: Json<PublishEvent>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            let mut event_id = md5(&thread_rng().gen::<u32>().to_string());
            loop {
                if (*EVENT_LIST).lock().unwrap().contains_key(&event_id) {
                        event_id = md5(&thread_rng().gen::<u32>().to_string());
                } else  {
                        (*EVENT_LIST).lock().unwrap().insert(event_id.clone(), Event{
                            event_id: event_id.clone(),
                            sponsor_name: sponsor_name.clone(),
                            event_name: new_event.event_name.clone(),
                            start_time: new_event.start_time.clone(),
                            end_time: new_event.end_time.clone(),
                            event_type: new_event.event_type,
                            event_introduction: new_event.event_introduction.clone(),
                            event_picture: new_event.event_picture.clone(),
                            event_capacity: new_event.event_capacity,
                            current_participants: 0,
                            left_tickets: new_event.left_tickets,
                            event_status: 0,
                            event_location: new_event.event_location.clone(),
                            update_type: 2,
                        });
                    break;
                }
            }
            match update_events() {
                Ok(_) => Ok(HttpResponse::Ok().json(EventIDRet { // 200 OK
                    event_id: event_id
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
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
pub fn get_all_events(
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            match sponsors::get_sponsor_events(&sponsor_name) {
                Ok(sponsor_event_list) => Ok(HttpResponse::Ok().json(EventsRet {
                    events: sponsor_event_list
                })),
                Err(e) => {
                    println!("{}", e);
                    Ok(HttpResponse::InternalServerError().finish())
                }
            }
            
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
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
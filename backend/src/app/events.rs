use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{Error, HttpRequest, HttpResponse, web::Data, web::Json};
use futures::{Future, future::result};
use serde::{Deserialize, Serialize};

use super::{ADMIN_ID, EVENT_LIST};
use super::update::{update_events};
use crate::db::events::Event;
use crate::utils::auth::{identify_sponsor, identify_user};

#[derive(Deserialize)]
pub struct QueryEvent {
    event_name: String,
}

#[derive(Deserialize)]
pub struct QueryEventByID {
    pub event_id: String,
}

#[derive(Serialize)]
pub struct EventsRet {
    pub events: Vec<Event>
}

#[derive(Serialize)]
struct BroadCastEventInfo {
    event_id: String,
    event_name: String,
    img_url: String,
    intro: String
}

#[derive(Serialize)]
struct BroadCastRet {
    list: Vec<BroadCastEventInfo>
}

#[allow(dead_code)]
pub fn get_broadcast_events(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(_) => {
            let mut list: Vec<BroadCastEventInfo> = vec![];
            let mut count = 0;
            for (_, event) in &(*EVENT_LIST.lock().unwrap()) {
                if (event.left_tickets > 0)
                    && (event.event_status == 1) {
                    list.push(BroadCastEventInfo {
                        event_id: event.event_id.clone(),
                        event_name: event.event_name.clone(),
                        img_url: event.event_picture.clone(),
                        intro: event.event_introduction.clone()
                    });
                    count += 1;
                    if count >= 6 {
                        break;
                    }
                }
            }
            Ok(HttpResponse::Ok().json(BroadCastRet {
                list: list
            }))
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[derive(Clone, Deserialize, Serialize)]
pub struct EventInfo {
    pub event_id: String,
    pub sponsor_name: String,
    pub event_name: String,
    pub start_time: String,
    pub end_time: String,
    pub event_type: i8,
    pub event_introduction: String,
    pub event_picture: String,
    pub event_capacity: i32,
    pub current_participants: i32,
    pub left_tickets: i32,
    pub event_status: i8,
    pub event_location: String,
    pub update_type: i8,
}

#[allow(dead_code)]
pub fn get_event_info(
    id: Identity,
    query_event: Json<QueryEventByID>
) -> impl Future<Item=HttpResponse, Error=Error> {
    if id.identity() == None {
        return result(Ok(HttpResponse::Unauthorized().finish())); // 401 Unauthorized
    }

    result(
        if (*EVENT_LIST).lock().unwrap().contains_key(&query_event.event_id) {
            Ok(HttpResponse::Ok().json(
                (*EVENT_LIST).lock().unwrap().get(&query_event.event_id)))
        } else {
            Ok(HttpResponse::UnprocessableEntity().json("Event does not exist."))
        }
    )
}

#[derive(Deserialize)]
pub struct AlterEvent {
    pub event_id: String,
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

#[allow(dead_code)]
pub fn alter_event_info(
    alter_event: Json<AlterEvent>,
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            match (*EVENT_LIST).lock().unwrap().get_mut(&alter_event.event_id) {
                Some(mut event) => {
                    if sponsor_name != event.sponsor_name {
                        return result(Ok(HttpResponse::Unauthorized().finish()));
                    }
                    event.event_name = alter_event.event_name.clone();
                    event.start_time = alter_event.start_time.clone();
                    event.end_time = alter_event.end_time.clone();
                    event.event_type = alter_event.event_type;
                    event.event_introduction = alter_event.event_introduction.clone();
                    event.event_picture = alter_event.event_picture.clone();
                    event.event_capacity = alter_event.event_capacity;
                    event.left_tickets = alter_event.left_tickets;
                    event.event_location = alter_event.event_location.clone();
                    event.update_type = 1;
                },
                None => return result(Ok(HttpResponse::UnprocessableEntity().json("No such event.")))
            }
            update_events(); // TODO: async
            Ok(HttpResponse::BadRequest().finish())
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}
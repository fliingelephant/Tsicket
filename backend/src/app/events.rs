use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{Error, HttpRequest, HttpResponse, web::Data, web::Json};
use futures::{Future, future::result};
use serde::{Deserialize, Serialize};

use super::{ADMIN_ID, EVENT_LIST};
use super::update::{update_events};
use crate::db::events::Event;
use crate::utils::auth::{identify_user};

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

#[allow(dead_code)]
pub fn get_event_info(
    (id, query_event):
        (Identity, Json<QueryEventByID>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    if id.identity() == None {
        return result(Ok(HttpResponse::Unauthorized().finish())); // 401 Unauthorized
    }

    //if *EVENT_LIST.lock().unwrap().entry(query_event.event_id)
    /*
    for event in &(*EVENT_LIST.lock().unwrap()) {
        if event.event_id == query_event.event_id {
            return result(Ok(HttpResponse::Ok().json(event.clone())));
        }
    }*/

    

    result(Ok(HttpResponse::BadRequest().finish()))
}

#[allow(dead_code)]
pub fn alter_event_info(
    (id, event_info):
        (Identity, Json<Event>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    /*
    result(match id.identity() {
        Some(name) => {
            let alter_event = event_info.into_inner();
            if alter_event.sponsor_name != name {
                return result(Ok(HttpResponse::Unauthorized().finish()));
            }
            for mut event in &(*EVENT_LIST.lock().unwrap()) {
                if (event.event_id == alter_event.event_id)
                    && (event.sponsor_name == name) {
                    event = &alter_event;
                }
            }
            let events = EVENT_LIST.lock().unwrap().clone();
            update_events(&events);
            Ok(HttpResponse::BadRequest().finish())
        },
        None => Ok(HttpResponse::Unauthorized().finish())
    })*/

    result(Ok(HttpResponse::BadRequest().finish()))
}
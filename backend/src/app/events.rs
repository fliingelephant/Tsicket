use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{Error, HttpRequest, HttpResponse, web::Data, web::Json};
use futures::{Future, future::result};
use serde::{Deserialize, Serialize};

use super::ADMIN_ID;
use super::EventState;
use crate::db::events::Event;

#[derive(Deserialize)]
pub struct QueryEvent {
    event_name: String,
}

#[derive(Serialize)]
pub struct EventsRet {
    pub events: Vec<Event>
}

#[inline]
pub fn book_event(
    (query_event, event_state, id):
        (Json<QueryEvent>, Data<Mutex<EventState>>, Identity)
) -> impl Future<Item=HttpResponse, Error=Error> {

    let mut event_state = event_state.lock().unwrap();

    result(Ok(HttpResponse::Ok().finish()))
    /*
    result(match event_state.event_list.get_mut(&query_event.event_name) {
        Some(mut event) => {
            if (event.left_tickets > 0) && (event.event_status == 1) {
                event.left_tickets-=1;
                if event.left_tickets == 0 {
                    event.event_status = 2;
                }
                Ok(HttpResponse::Ok().finish()) // 200 Ok
            } else {
                Ok(HttpResponse::BadRequest().finish()) // 400 Bad Request
            }
        },
        None => Ok(HttpResponse::BadRequest().finish()) // 400 Bad Request
    })*/
}

#[inline]
pub fn get_all_events(
    (event_state, id):
        (Data<Mutex<EventState>>, Identity)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(id) => {
            if (id == *ADMIN_ID) {
                // TODO
                Ok(HttpResponse::NotImplemented().finish())
            } else {
                Ok(HttpResponse::Unauthorized().finish())
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[inline]
pub fn get_broadcast_events(
    (event_state, id):
        (Data<Mutex<EventState>>, Identity)
) -> impl Future<Item=HttpResponse, Error=Error> {
    if id.identity() == None {
        return result(Ok(HttpResponse::Unauthorized().finish())); // 401 Unauthorized
    }
    
    let mut event_state = event_state.lock().unwrap();
    // TODO
    result(Ok(HttpResponse::NotImplemented().finish()))
}


pub fn get_event_info(
    (event_state, id, query_event):
        (Data<Mutex<EventState>>, Identity, Json<QueryEvent>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    if (id.identity() == None) {
        return result(Ok(HttpResponse::Unauthorized().finish())); // 401 Unauthorized
    }

    
    let mut event_state = event_state.lock().unwrap();
    for event in &event_state.event_list {
        if (event.event_name == query_event.event_name) {
            return result(Ok(HttpResponse::Ok().json(event.clone())));
        }
    }

    result(Ok(HttpResponse::BadRequest().finish()))
}
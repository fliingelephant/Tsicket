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

pub fn book_event(
    (query_event, event_state, id):
        (Json<QueryEvent>, Data<Mutex<EventState>>, Identity)
) -> impl Future<Item=HttpResponse, Error=Error> {

    let mut event_state = event_state.lock().unwrap();
    result(match event_state.event_list.get(&query_event.event_name) {
        Some(event) => {
            //if (event.left_tickets > 0) && (event.status == )
            //TODO
            Ok(HttpResponse::NotImplemented().finish())
        },
        None => Ok(HttpResponse::BadRequest().finish()) // 400 Bad Request
    })
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
    if (id.identity() == None) {
        return result(Ok(HttpResponse::Unauthorized().finish())); // 401 Unauthorized
    }
    
    let mut event_state = event_state.lock().unwrap();
    // TODO
    result(Ok(HttpResponse::NotImplemented().finish()))
}

#[inline]
pub fn get_event_info(
    (event_state, id, query_event):
        (Data<Mutex<EventState>>, Identity, Json<QueryEvent>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    if (id.identity() == None) {
        return result(Ok(HttpResponse::Unauthorized().finish())); // 401 Unauthorized
    }
    
    let mut event_state = event_state.lock().unwrap();
    result(match event_state.event_list.get(&query_event.event_name) {
        Some(event) => {
            // TODO
            Ok(HttpResponse::NotImplemented().finish())
        },
        None => Ok(HttpResponse::BadRequest().finish()) // 400 Bad Request
    })
}
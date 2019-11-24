use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_web::{
    Error,
    HttpRequest, 
    HttpResponse,
    web::Data,
    web::Json
};
use futures::{Future, future::result};
use md5::compute;
use serde::Deserialize;

use crate::app::ADMIN_ID;
use crate::app::ADMIN_PASSWORD_WITH_SALT;
use super::events::EventsRet;
use super::EventState;
use super::EVENT_LIST;
use crate::db::events;
use crate::db::events::Event;

#[inline]
fn md5_with_salt(id: &String, raw_password: &String) -> String {
    format!("{:x}", md5::compute(raw_password.to_owned() + id))
}

#[derive(Debug, Deserialize)]
pub struct LoginAdmin {
    pub admin_id: String,
    pub password: String,
}

#[inline]
pub fn login(
    id: Identity,
    login_admin: Json<LoginAdmin>
) -> impl Future<Item=HttpResponse, Error=Error> {
    let password = md5_with_salt(&login_admin.admin_id, &login_admin.password);

    if (login_admin.admin_id == *ADMIN_ID) && (password == *ADMIN_PASSWORD_WITH_SALT) {
        id.remember(login_admin.admin_id.to_owned());
        result(Ok(HttpResponse::Ok().json("Admin login success.")))
    } else {
        result(Ok(HttpResponse::UnprocessableEntity().json("Wrong!")))
    }
}

#[inline]
pub fn logout(
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    id.forget();
    result(Ok(HttpResponse::Ok().finish()))
}

pub fn get_all_events(
    (id):
        (Identity),
) -> impl Future<Item=HttpResponse, Error=Error> {
    if let Some(admin_id) = id.identity() {
        if (admin_id == *ADMIN_ID) {
            let mut all_event_list: Vec<Event> = vec![];
            
            //let mut state = state.lock().unwrap();
            //println!("{}",state.event_list.len());
            for event in &(*EVENT_LIST.lock().unwrap()) {
                all_event_list.push(event.clone())
            }
            result(Ok(HttpResponse::Ok().json(EventsRet {
                    events: all_event_list
                }
            )))
        } else {
            result(Ok(HttpResponse::Unauthorized().finish()))
        }
    } else {
        result(Ok(HttpResponse::Unauthorized().finish()))
    }
}
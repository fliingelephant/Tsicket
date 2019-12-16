use actix_identity::{Identity};
use actix_web::{
    Error, 
    HttpRequest,
    HttpResponse,
    web::Json
};
use futures::{Future, future::result};
use md5::compute;
use serde::{Deserialize, Serialize};

use super::{ADMIN_ID, ADMIN_PASSWORD_WITH_SALT, EVENT_LIST};
use crate::db::admins;

use crate::utils::auth::{identify_admin};

#[inline]
fn md5_with_salt(id: &String, raw_password: &String) -> String {
    format!("{:x}", compute(raw_password.to_owned() + id))
}

#[derive(Deserialize)]
pub struct LoginAdmin {
    pub admin_id: String,
    pub password: String,
}

#[allow(dead_code)]
pub fn login(
    id: Identity,
    login_admin: Json<LoginAdmin>
) -> impl Future<Item=HttpResponse, Error=Error> {
    let password = md5_with_salt(&login_admin.admin_id, &login_admin.password);

    if (login_admin.admin_id == *ADMIN_ID) && (password == *ADMIN_PASSWORD_WITH_SALT) {
        id.remember("0".to_owned() + &login_admin.admin_id);
        result(Ok(HttpResponse::Ok().json("Admin login success."))) // 200 OK
    } else {
        result(Ok(HttpResponse::UnprocessableEntity().json("Failed to log in."))) // 422 Unprocessable Entity
    }
}

#[allow(dead_code)]
pub fn logout(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            id.forget();
            Ok(HttpResponse::Ok().finish()) // 200 OK
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
pub struct EventInfoRetToAdmin {
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
    pub event_time: String,
}

#[derive(Serialize)]
struct AllEventsRet {
    events: Vec<EventInfoRetToAdmin>
}

#[allow(dead_code)]
pub fn get_all_events(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let mut all_event_list: Vec<EventInfoRetToAdmin> = vec![];
            admins::get_all_events(&mut all_event_list).unwrap();
            Ok(HttpResponse::Ok().json(AllEventsRet { // 200 OK
                events: all_event_list
            }))
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn review_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let event_id = req.match_info().query("event_id").to_string();
            match (*EVENT_LIST).lock().unwrap().get_mut(&event_id) {
                Some(mut event) => {
                    if event.event_status == 0 {
                        // TODO: 判断时间
                        event.event_status = 1;
                        event.update_type = 1;
                        Ok(HttpResponse::Ok().finish())
                    } else {
                        Ok(HttpResponse::UnprocessableEntity().json("Event status error!"))
                    }
                }
                None => Ok(HttpResponse::UnprocessableEntity().json("Event status error!"))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn cancel_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let event_id = req.match_info().query("event_id").to_string();
            match (*EVENT_LIST).lock().unwrap().get_mut(&event_id) {
                Some(mut event) => {
                    if event.event_status == 4 {
                        // TODO: 判断时间
                        event.event_status = 4;
                        event.update_type = 1;
                    }
                    Ok(HttpResponse::Ok().finish())
                }
                None => Ok(HttpResponse::UnprocessableEntity().json("Event status error!"))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
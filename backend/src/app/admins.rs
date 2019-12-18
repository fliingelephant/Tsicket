use actix_identity::{Identity};
use actix_web::{
    Error, 
    HttpRequest,
    HttpResponse,
    web::Json,
    web::Query
};
use futures::{Future, future::result};
use md5::compute;
use serde::{Deserialize, Serialize};

use super::{ADMIN_ID, ADMIN_PASSWORD_WITH_SALT, EVENT_LIST};
use super::events::{QueryEventByID};
use super::update::{update_events};
use crate::db::{admins, sponsors};

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

#[derive(Serialize, Clone)]
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
            let mut events =  (*EVENT_LIST).lock().unwrap();
            let event_status: i8;
            match events.get(&event_id) {
                Some(event) => {
                    event_status = event.event_status % 10;
                }
                None => {
                    event_status = -1;
                }
            }
            match event_status {
                -1 => Ok(HttpResponse::UnprocessableEntity().finish()),
                0 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status += 1;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    
                    Ok(HttpResponse::Ok().finish())
                }
                _ => Ok(HttpResponse::UnprocessableEntity().finish())
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn cancel_review_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let event_id = req.match_info().query("event_id").to_string();
            let mut events =  (*EVENT_LIST).lock().unwrap();
            let event_status: i8;
            match events.get(&event_id) {
                Some(event) => {
                    event_status = event.event_status % 10;
                }
                None => {
                    event_status = -1;
                }
            }
            match event_status {
                -1 => Ok(HttpResponse::UnprocessableEntity().finish()),
                1 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status += 3;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    
                    Ok(HttpResponse::Ok().finish())
                }
                2 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status += 2;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    
                    Ok(HttpResponse::Ok().finish())
                }
                _ => Ok(HttpResponse::UnprocessableEntity().finish())
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
            let mut events =  (*EVENT_LIST).lock().unwrap();
            let event_status: i8;
            match events.get(&event_id) {
                Some(event) => {
                    event_status = event.event_status % 10;
                }
                None => {
                    event_status = -1;
                }
            }
            match event_status {
                -1 => {
                    match admins::cancel_event(&event_id) {
                        Ok(_) => Ok(HttpResponse::Ok().finish()),
                        Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
                    }
                }
                1 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status += 3;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    
                    Ok(HttpResponse::Ok().finish())
                }
                2 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status += 2;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    
                    Ok(HttpResponse::Ok().finish())
                }
                _ => Ok(HttpResponse::UnprocessableEntity().finish())
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
pub struct AdApplicationsRet {
    pub events: Vec<EventInfoRetToAdmin>
}

#[allow(dead_code)]
pub fn get_ad_applications(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let events = (*EVENT_LIST).lock().unwrap();
            let mut events_in_application = vec![];
    
            for event in events.values() {
                if event.event_status < 10 {
                    events_in_application.push(EventInfoRetToAdmin {
                        event_id: event.event_id.clone(),
                        sponsor_name: event.sponsor_name.clone(),
                        event_name: event.event_name.clone(),
                        start_time: event.start_time.clone(),
                        end_time: event.end_time.clone(),
                        event_type: event.event_type,
                        event_introduction: event.event_introduction.clone(),
                        event_picture: event.event_picture.clone(),
                        event_capacity: event.event_capacity,
                        current_participants: event.current_participants,
                        left_tickets: event.left_tickets,
                        event_status: event.event_status,
                        event_location: event.event_location.clone(),
                        event_time: event.event_time.clone(),
                    });
                }
            }
            Ok(HttpResponse::Ok().json(AdApplicationsRet {
                events: events_in_application
            }))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
pub fn allow_advertise(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let event_id = req.match_info().query("event_id").to_string();
            let mut events = (*EVENT_LIST).lock().unwrap();
            let ad_status: i8;
            match events.get(&event_id) {
                Some(event) => {
                    ad_status = event.event_status / 10;
                }
                None => {
                    ad_status = -1;
                }
            }
            match ad_status {
                -1 => Ok(HttpResponse::UnprocessableEntity().finish()),
                0 => Ok(HttpResponse::UnprocessableEntity().json("Event not in application for advertisement.")),
                1 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status += 10;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    Ok(HttpResponse::Ok().finish())
                }
                2 => Ok(HttpResponse::UnprocessableEntity().json("Event already under advertisement.")),
                _ => Ok(HttpResponse::InternalServerError().finish())
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
pub fn cancel_advertise(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let event_id = req.match_info().query("event_id").to_string();
            let mut events = (*EVENT_LIST).lock().unwrap();
            let ad_status: i8;
            match events.get(&event_id) {
                Some(event) => {
                    ad_status = event.event_status / 10;
                }
                None => {
                    ad_status = -1;
                }
            }
            match ad_status {
                -1 => Ok(HttpResponse::UnprocessableEntity().finish()),
                0 => Ok(HttpResponse::UnprocessableEntity().json("Event not in application for advertisement.")),
                1 => Ok(HttpResponse::UnprocessableEntity().json("Event just in application for advertisement.")),
                2 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status -= 10;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    Ok(HttpResponse::Ok().finish())
                }
                _ => Ok(HttpResponse::InternalServerError().finish())
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
pub fn get_sponsor_info(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let sponsor_name = req.match_info().query("sponsor_name").to_string();
            match sponsors::get_info_by_name(&sponsor_name) {
                Ok(sponsor) => Ok(HttpResponse::Ok().json(sponsor)),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn get_all_sponsor_info(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_admin(&id) {
        Ok(_) => {
            let sponsor_name = req.match_info().query("sponsor_name").to_string();
            /*match sponsors::get_
                Ok(sponsor) => Ok(HttpResponse::Ok().json(sponsor)),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }*/
            Ok(HttpResponse::NotImplemented().finish())
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
use std::fs;
use std::io::{Write};
use std::path::{PathBuf};
use std::sync::{Mutex};

use actix_identity::{Identity};
use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{
    error,
    Error,
    HttpRequest,
    HttpResponse,
    web,
    web::Json};
use chrono::prelude::Local;
use futures::{Stream, Future, future::result};
use futures::future::{err, Either};
use md5::compute;
use rand::{thread_rng, Rng};
use serde::{Deserialize, Serialize};

use super::events::{EventsRet, QueryEventByID};
use crate::db::events::{Event};
use crate::db::sponsors;
use crate::db::moment;
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
    req: HttpRequest,
    id: Identity,
) -> impl Future<Item=HttpResponse, Error=Error> {
    println!("{:?}", req.head());
    println!("{:?}", req.headers());

    result(match identify_sponsor(&id) {
        Ok(_) => {
            id.forget();
            Ok(HttpResponse::Ok().finish()) // 200 OK
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn register(
    id: Identity,
    register_sponsor: Json<RegisterSponsor>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match sponsors::sponsor_register(
        &register_sponsor.id,
        &register_sponsor.sponsorname,
        &"http://2019-a18.iterator-traits.com/apis/sponsors/pic/default_avatar.jpg".to_string(),
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
    pub event_time: String,
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
                            event_time: new_event.event_time.clone(),
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
pub fn cancel_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
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
            // TODO: 检查是否是主办方自己的活动
            match event_status {
                -1 => Ok(HttpResponse::UnprocessableEntity().finish()),
                1 => {
                    let mut event = events.get_mut(&event_id).unwrap();
                    event.event_status += 2;
                    event.update_type = 1;
                    drop(events);
                    update_events();
                    
                    Ok(HttpResponse::Ok().finish())
                }
                2 => {
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
pub fn advertise_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            let event_id = req.match_info().query("event_id").to_string();
            let mut events = (*EVENT_LIST).lock().unwrap();
            let status: i8;
            let ad_status: i8;
            match events.get(&event_id) {
                Some(event) => {
                    status = event.event_status % 10;
                    ad_status = event.event_status / 10;
                }
                None => {
                    status = -1;
                    ad_status = -1;
                }
            }
            match status {
                -1 => Ok(HttpResponse::UnprocessableEntity().json("Event does not exist; Otherwise has been canceled or finished its booking.")),
                0 => Ok(HttpResponse::UnprocessableEntity().json("Event has not yet been reviewed.")),
                1 => {
                    match ad_status {
                        0 => {
                            let mut event = events.get_mut(&event_id).unwrap();
                            event.event_status = 11;
                            event.update_type = 1;
                            drop(events);
                            update_events();
                            Ok(HttpResponse::Ok().finish())
                        }
                        1 => Ok(HttpResponse::UnprocessableEntity().json("Already in application for advertisement.")),
                        2 => Ok(HttpResponse::UnprocessableEntity().json("Already advertising now.")),
                        _ => Ok(HttpResponse::InternalServerError().finish())
                    }
                }
                2 => {
                    match ad_status {
                        0 => {
                            let mut event = events.get_mut(&event_id).unwrap();
                            event.event_status = 12;
                            event.update_type = 1;
                            drop(events);
                            update_events();
                            Ok(HttpResponse::Ok().finish())
                        }
                        1 => Ok(HttpResponse::UnprocessableEntity().json("Already in application for advertisement.")),
                        2 => Ok(HttpResponse::UnprocessableEntity().json("Already advertising now.")),
                        _ => Ok(HttpResponse::InternalServerError().finish())
                    }
                }
                _ => Ok(HttpResponse::UnprocessableEntity().json("Event does not exist; Otherwise has been canceled or finished its booking.")),
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
pub fn cancel_advertise_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            let event_id = req.match_info().query("event_id").to_string();
            let mut events = (*EVENT_LIST).lock().unwrap();
            let status: i8;
            let ad_status: i8;
            match events.get(&event_id) {
                Some(event) => {
                    status = event.event_status % 10;
                    ad_status = event.event_status / 10;
                }
                None => {
                    status = -1;
                    ad_status = -1;
                }
            }
            match status {
                -1 => Ok(HttpResponse::UnprocessableEntity().json("No such event.")),
                0 => Ok(HttpResponse::UnprocessableEntity().json("Event has not yet been reviewed.")),
                1 => {
                    match ad_status {
                        0 => Ok(HttpResponse::UnprocessableEntity().json("Not in application for advertisement now.")),
                        1 => {
                            let mut event = events.get_mut(&event_id).unwrap();
                            event.event_status = 1;
                            event.update_type = 1;

                            drop(events);
                            update_events();

                            Ok(HttpResponse::Ok().finish())
                        }
                        2 => Ok(HttpResponse::UnprocessableEntity().json("Already advertising now.")),
                        _ => Ok(HttpResponse::UnprocessableEntity().json("Something seems to be wrong."))
                    }
                }
                2 => {
                    match ad_status {
                        0 => Ok(HttpResponse::UnprocessableEntity().json("Not in application for advertisement now.")),
                        1 => {
                            let mut event = events.get_mut(&event_id).unwrap();
                            event.event_status = 2;
                            event.update_type = 1;

                            drop(events);
                            update_events();

                            Ok(HttpResponse::Ok().finish())
                        }
                        2 => Ok(HttpResponse::UnprocessableEntity().json("Already advertising now.")),
                        _ => Ok(HttpResponse::UnprocessableEntity().json("Something seems to be wrong."))
                    }
                }
                _ => Ok(HttpResponse::UnprocessableEntity().json("Event has been canceled or finished its booking.")),
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
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
pub fn get_sponsor_info(
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

pub fn save_file(
    field: Field,
    file_name: &String
) -> impl Future<Item=String, Error=Error> {
    println!("{:?}", field.content_type());
    let mut suffix = field.content_type().subtype().to_string();
    if suffix == "jpeg" {
        suffix = "jpg".to_string();
    }
    let name_assigned_by_server = file_name.to_owned() + "." + &suffix;
    let path = "../../../static/".to_string() + &file_name + "." + &suffix;

    let file = match fs::File::create(path) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        println!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| {
                    match e {
                        error::BlockingError::Error(e) => e,
                        error::BlockingError::Canceled => MultipartError::Incomplete,
                    }
                })
            })
            .map(move |(_, _)| name_assigned_by_server)
            .map_err(|e| {
                println!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

#[inline]
pub fn two_terms_md5(term_a: String, term_b: &str) -> String {
    format!("{:x}_{:x}", compute(term_a.to_owned() + term_b), compute(term_b))
}

#[derive(Serialize)]
pub struct FileURLRet {
    file_url: String
}

#[allow(dead_code)]
pub fn update_pic(
    id: Identity,
    multipart: Multipart,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    let file_name: String;
    let name_assigned_by_sponsor = req.match_info().query("filename");
    if let Ok(sponsor_name) = identify_sponsor(&id) {
        file_name = two_terms_md5(sponsor_name, name_assigned_by_sponsor);
    } else {
        file_name = "default".to_string();
    }

        multipart
            .map_err(error::ErrorInternalServerError)
            .map(move |field| save_file(field, &file_name).into_stream())
            .flatten()
            .collect()
            .map(|name_assigned_by_server| {
                HttpResponse::Ok().json(FileURLRet {
                    file_url: "http://2019-a18.iterator-traits.com/apis/sponsors/pic/".to_string() + &name_assigned_by_server[0]
                })
            })
            .map_err(|e| {
                println!("failed: {}", e);
                e
            })
}

#[allow(dead_code)]
pub fn get_pic(
    req: HttpRequest
) -> actix_web::Result<actix_files::NamedFile> {
    let path: PathBuf = ("../../../static/".to_string() + &req.match_info().query("filename")).parse().unwrap();
    println!("serving file: {:?}", path);

    Ok(actix_files::NamedFile::open(path)?)
}

#[derive(Deserialize)]
pub struct Moment {
    pub event_name: String,
    pub text: String,
    pub pictures: Vec<String>
}

#[allow(dead_code)]
pub fn publish_moment(
    id: Identity,
    moment: Json<Moment>,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            let event_id = req.match_info().query("event_id").to_string();
            // TODO: 检查是否符合活动和发布者对应
            let now = Local::now().to_string();
            match moment::publish_moment(&sponsor_name, &event_id, &md5(&(sponsor_name.clone() + &event_id + &now)),
             &moment.event_name, &moment.text, &moment.pictures) {
                Ok(_) => Ok(HttpResponse::Ok().finish()),
                Err(e) => {
                    println!("{}", e);
                    Ok(HttpResponse::UnprocessableEntity().json(e))
                }
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[derive(Deserialize)]
pub struct EventPost {
    pub text: String
}

#[allow(dead_code)]
pub fn publish_post(
    id: Identity,
    post: Json<EventPost>,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_sponsor(&id) {
        Ok(sponsor_name) => {
            let event_id = req.match_info().query("event_id").to_string();

            Ok(HttpResponse::NotImplemented().finish())
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

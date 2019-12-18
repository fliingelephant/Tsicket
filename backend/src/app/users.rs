use std::env;
use std::fs;
use std::io::{Write};
use std::mem::drop;
use std::path::{PathBuf};

use actix_multipart::{Field, Multipart, MultipartError};
use actix_identity::Identity;
use actix_web::{client, error, Error, http::Method, HttpRequest, HttpResponse, ResponseError, web, web::Data, web::Json, web::Query};
use futures::{Future, future::result, Stream};
use futures::future::{err, Either};
use md5::compute;
use reqwest::{get, Client};
use serde::{Deserialize, Serialize};
use actix_service::ServiceExt;

use super::{APP_ID, EVENT_LIST, RECORD, SECRET};
use super::sponsors::{QuerySponsor, QuerySponsorByID};
use super::events::{QueryEvent, QueryEventByID, EventsRet};
use crate::db::events;
use crate::db::moment;
use crate::db::records::{Record};
use crate::db::sponsors;
use crate::db::users;
use crate::utils::auth::{identify_user};
use super::update::{update_records, update_events};

#[derive(Deserialize)]
pub struct WechatLoginInfo {
    code: String,
}

#[derive(Serialize)]
pub struct WechatLoginReturn {
    openid: String,
}

#[derive(Debug, Deserialize)]
pub struct MPLoginInfo {
    openid: String,
    session_key: String,
}

#[derive(Serialize)]
pub struct MPTHUInfo {
    token: String,
}


#[derive(Deserialize)]
pub struct ErrorInTsinghuaUser {
    code: i32,
    message: String,
}

#[derive(Deserialize)]
pub struct TsinghuaUser {
    card: String,
    name: String,
    department: String,
    cell: String,
    mail: String,
    //error: ErrorInTsinghuaUser,
}

#[derive(Deserialize)]
pub struct TsinghuaInfo {
    user: TsinghuaUser,
}

#[derive(Serialize)]
pub struct WechatTHUReturn {
    tsinghuaid: String,
}

#[derive(Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct QueryList {
    pub index: usize,
}

#[allow(dead_code)]
pub fn login(
    id: Identity,
    info: Json<WechatLoginInfo>
) -> impl Future<Item=HttpResponse, Error=Error> {
    let url = format!("https://api.weixin.qq.com/sns/jscode2session?\
        appid={appid}&secret={secret}&js_code={code}&grant_type=authorization_code",
        appid = env::var("APP_ID").unwrap(), secret = env::var("SECRET").unwrap(), code = info.code);

    let resp = reqwest::get(&url);
    
    let content = resp.unwrap().json::<MPLoginInfo>();
    result(match content {
        Ok(text)=> {

            id.remember("2".to_owned() + &text.openid);
            if let Some(id) = id.identity() {
                println!("{}", id);
            }
            match users::add_user(&text.openid) {
                Ok(_) => Ok(HttpResponse::Ok().json(WechatLoginReturn {
                                openid: text.openid.clone(),
                            })),
                Err(e) => {
                    println!("{}", e);
                    Ok(HttpResponse::Ok().json(WechatLoginReturn {
                        openid: text.openid.clone(),
                    }))
                }
            }
        },
        Err(e) => {
            println!("{}", e.to_string());
            Ok(HttpResponse::UnprocessableEntity().json("Wrong code"))
        }
    })
}

#[derive(Deserialize)]
pub struct WechatTHUInfo {
    token: String
}

#[allow(dead_code)]
pub fn bind_tsinghua_id(
    id: Identity,
    info: Json<WechatTHUInfo>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let client = reqwest::Client::new();
            let resp = client
                .post("https://alumni-test.iterator-traits.com/fake-id-tsinghua-proxy/api/user/session/token")
                .json(&MPTHUInfo {
                    token: info.token.clone(),
                })
                .send();

            let content = resp.unwrap().json::<TsinghuaInfo>();
            match content {
                Ok(text)=> {
                    match users::set_tsinghua_id(&openid, &text.user.card) {
                        Ok(_) => {}
                        Err(e) => {println!("{}", e);}
                    }
                    Ok(HttpResponse::Ok().json(WechatTHUReturn {
                            tsinghuaid: text.user.card.clone(),
                    }))
                },
                Err(e) => {
                    println!("{:?}", e.to_string());
                    Ok(HttpResponse::UnprocessableEntity().json("Wrong code!"))
                }
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
pub fn get_tsinghua_id(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match users::get_tsinghua_id(&openid) {
                Ok(tsinghua_id) => Ok(HttpResponse::Ok().json(WechatTHUReturn {
                    tsinghuaid: tsinghua_id
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[derive(Serialize)]
struct UserInfo {
    follow: usize,
    history: usize,
    like: usize,
    tsinghuaid: String,
}

#[derive(Serialize)]
struct UserInfoWithoutTHUID {
    follow: usize,
    history: usize,
    like: usize,
}

pub fn get_personal_info(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    println!("{:?}", req.headers());

    result(match identify_user(&id) {
        Ok(openid) => {
            let follow = users::get_user_follows(&openid).unwrap().len();
            let like = users::get_user_likes(&openid).unwrap().len();
            let history = users::get_user_records(&openid).unwrap().len();
            match users::get_tsinghua_id(&openid) {
                Ok(tsinghua_id) => Ok(HttpResponse::Ok().json(UserInfo{
                    follow: follow,
                    history: history,
                    like: like,
                    tsinghuaid: tsinghua_id
                })),
                Err(_) => Ok(HttpResponse::Ok().json(UserInfoWithoutTHUID{
                    follow: follow,
                    history: history,
                    like: like,
                }))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
struct FollowRet {
    follow: bool,
}

#[allow(dead_code)]
pub fn check_follow(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let sponsor_name = req.match_info().query("sponsor_name").to_string();
            match users::check_user_follow(&openid, &sponsor_name) {
                Ok(flag) => Ok(HttpResponse::Ok().json(FollowRet { // 200 Ok
                    follow: flag,
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn follow_or_unfo(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let sponsor_name = req.match_info().query("sponsor_name").to_string();
            match users::check_user_follow(&openid, &sponsor_name) {
                Ok(flag) => {
                    if flag == false {
                        match users::set_user_follow(&openid, &sponsor_name) {
                            Ok(_) => Ok(HttpResponse::Ok().json(FollowRet { // 200 OK
                                follow: true
                            })),
                            Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
                        }
                    } else {
                        match users::cancel_user_follow(&openid, &sponsor_name) {
                            Ok(_) => Ok(HttpResponse::Ok().json(FollowRet { // 200 OK
                                follow: false
                            })),
                            Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
                        }
                    }
                }
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn follow(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let sponsor_name = req.match_info().query("sponsor_name").to_string();
            match users::set_user_follow(&openid, &sponsor_name) {
                Ok(_) => Ok(HttpResponse::Ok().finish()), // 200 OK
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn unfo(
    (id, query_sponsor):
    (Identity, Json<QuerySponsor>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match users::cancel_user_follow(&openid, &query_sponsor.sponsor_name) {
                Ok(_) => Ok(HttpResponse::Ok().finish()), // 200 OK
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
struct SponsorInfo {
    avatar_url: String,
    follow: bool,
    id: String,
    name: String,
    to_start: usize,
    history: usize
}

#[derive(Serialize)]
struct FollowedSponsorInfo {
    avatar_url: String,
    id: String,
    name: String,
    to_start: usize,
    history: usize
}

#[derive(Serialize)]
struct FollowListRet {
    more: bool,
    list: Vec<FollowedSponsorInfo>,
}

pub fn get_follow_list(
    (id, Query(query_list)):
    (Identity, Query<QueryList>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match users::get_user_follows(&openid) {
                Ok(name_list) => {
                    let index = query_list.index;
                    let mut list: Vec<FollowedSponsorInfo> = vec![];

                    let more: bool;
                    let end: usize;
                    if index + 15 >= name_list.len() {  // if no more sponsors followed
                        more = false;
                        end = name_list.len();
                    } else {
                        more = true;
                        end = index + 15;
                    }
                    for i in index..end {
                        match sponsors::get_info_by_name(&name_list[i]) {
                            Ok(sponsor) => {
                                let mut history_num = 0;
                                let mut to_start_num = 0;
                                let events = sponsors::get_sponsor_events(&sponsor.sponsor_name).unwrap();
                                for event in events {
                                    let status = event.event_status % 10;
                                    match status {
                                        3 => {
                                            history_num += 1;
                                        }
                                        1 => {
                                            to_start_num += 1;
                                        }
                                        2 => {
                                            to_start_num += 1;
                                        }
                                        _ => {}
                                    }
                                }
                                list.push(FollowedSponsorInfo {
                                    avatar_url: sponsor.head_portrait.clone(),
                                    history: history_num,
                                    id: sponsor.id.clone(),
                                    name: name_list[i].clone(),
                                    to_start: to_start_num,
                                })
                            },
                            Err(_) => {
                                break;
                            }
                        }
                    }
                    Ok(HttpResponse::Ok().json(FollowListRet { // 200 OK
                        more: more,
                        list: list,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
struct LikeRet {
    like: bool,
}

pub fn check_like(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let event_id = req.match_info().query("event_id").to_string();
            match users::check_user_like(&openid, &event_id) {
                Ok(flag) => Ok(HttpResponse::Ok().json(LikeRet { // 200 Ok
                    like: flag,
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn like(
    (id, query_event):
    (Identity, Json<QueryEventByID>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match users::set_user_like(&openid, &query_event.event_id) {
                Ok(_) => Ok(HttpResponse::Ok().finish()), // 200 OK
                Err(_) => Ok(HttpResponse::UnprocessableEntity().json("Already liking.")) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn dislike(
    (id, query_event):
    (Identity, Json<QueryEventByID>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match users::cancel_user_like(&openid, &query_event.event_id) {
                Ok(_) => Ok(HttpResponse::Ok().finish()), // 200 OK
                Err(_) => Ok(HttpResponse::UnprocessableEntity().json("Already not liking.")) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn like_or_dislike(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let event_id = req.match_info().query("event_id").to_string();
            match users::check_user_like(&openid, &event_id) {
                Ok(flag) => {
                    if flag == false {
                        match users::set_user_like(&openid, &event_id) {
                            Ok(_) => Ok(HttpResponse::Ok().json(LikeRet {
                                like: true
                            })), // 200 OK
                            Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
                        }
                    } else {
                        match users::cancel_user_like(&openid, &event_id) {
                            Ok(_) => Ok(HttpResponse::Ok().json(LikeRet {
                                like: false
                            })), // 200 OK
                            Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
                        }
                    }
                }
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
pub struct EventInfo {
    event_id: String,
    event_name: String,
    img_url: String,
    intro: String,
    start_time: String,
}

#[derive(Serialize)]
pub struct LikeListRet {
    more: bool,
    list: Vec<EventInfo>,
}

#[allow(dead_code)]
pub fn get_like_list(
    id: Identity,
    query_list: Query<QueryList>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match users::get_user_likes(&openid) {
                Ok(id_list) => {
                    let index = query_list.index;
                    let mut list: Vec<EventInfo> = vec![];

                    let more: bool;
                    let end: usize;
                    if index + 6 >= id_list.len() {  // if no more events liked
                        more = false;
                        end = id_list.len();
                    } else {
                        more = true;
                        end = index + 6;
                    }
                    for i in index..end {
                        match events::get_info_by_id(&id_list[i]) {
                            Ok(event) => {
                                list.push(EventInfo {
                                    event_id: id_list[i].clone(),
                                    event_name: event.event_name.clone(),
                                    img_url: event.event_picture.clone(),
                                    intro: event.event_introduction.clone(),
                                    start_time: event.start_time.clone(),
                                })
                            },
                            Err(_) => {
                                break;
                            }
                        }
                    }
                    Ok(HttpResponse::Ok().json(LikeListRet { // 200 OK
                        more: more,
                        list: list,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
pub struct SuccessRet {
    pub success: bool
}

#[inline]
pub fn book_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let event_id = req.match_info().query("event_id").to_string();
            match (*EVENT_LIST).try_lock() {
                Ok(a) => {println!("OKOKOK!");drop(a);}
                Err(e) => {println!("{:?}", e);}
            }
            let mut events = (*EVENT_LIST).lock().unwrap();
            match events.get_mut(&event_id) {
                Some(mut event) => {
                    println!("{} is booking {}", openid, event_id);
                    if (event.event_status % 10 == 2) && (event.left_tickets > 0) {
                        event.left_tickets -= 1;
                        event.update_type = 1;
                        let index_str = event_id + "_" + &openid;
                        let success: bool;
                        match (*RECORD).try_lock() {
                            Ok(a) => {println!("OKOKOK!");drop(a);}
                            Err(e) => {println!("{:?}", e);}
                        }
                        let mut records = (*RECORD).lock().unwrap();
                        success = match records.get(&index_str) {
                            Some(_) => false,
                            None => true,
                        };
                        println!("HERE!!{:?}",success);
                        if success == true {
                            records.insert(index_str.clone(), Record {
                                // TODO: 改变record_id
                                record_id: index_str,
                                event_id: event.event_id.clone(),
                                sponsor_name: event.sponsor_name.clone(),
                                user_id: openid,
                                start_time: event.start_time.clone(),
                                end_time: event.end_time.clone(),
                                update_type: 1
                            });
                        }
                        drop(records);
                        drop(events);
                        match update_events() {
                            Ok(_) => {}
                            Err(e) => {println!("{}",e);}
                        }
                        println!("HERE!!updated events");
                        match update_records() {
                            Ok(_) => {}
                            Err(e) => {println!("{}",e);}
                        }
                        Ok(HttpResponse::Ok().json(SuccessRet {
                            success: success
                        }))
                    } else {
                        Ok(HttpResponse::Ok().json(SuccessRet {
                            success: false
                        }))
                    }
                }
                None => {
                    drop(events);
                    Ok(HttpResponse::UnprocessableEntity().finish())
                }
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
    result(match identify_user(&id) {
        Ok(openid) => {
            let sponsor_name = req.match_info().query("sponsor_name").to_string();
            let follow = users::check_user_follow(&openid, &sponsor_name).unwrap();
            let mut to_start_num = 0;
            let mut history_num = 0;

            let events = sponsors::get_sponsor_events(&sponsor_name).unwrap();
            for event in events {
                let status = event.event_status % 10;
                match status {
                    3 => {
                        history_num += 1;
                    }
                    1 => {
                        to_start_num += 1;
                    }
                    2 => {
                        to_start_num += 1;
                    }
                    _ => {}
                }
            }

            match sponsors::get_info_by_name(&sponsor_name) {
                Ok(sponsor) => Ok(HttpResponse::Ok().json(SponsorInfo { // 200 OK
                    avatar_url: sponsor.head_portrait,
                    follow: follow,
                    history: history_num,
                    id: sponsor.id,
                    name: sponsor.sponsor_name,
                    to_start: to_start_num,
                })),
                Err(e) => {
                    println!("{}", e);
                    Ok(HttpResponse::InternalServerError().finish()) // 500 Internal Server Error
                }
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[derive(Serialize)]
pub struct EventsRetToUser {
    pub list: Vec<EventInfo>
}

#[allow(dead_code)]
pub fn get_available_enrolled_events(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let mut events: Vec<EventInfo> = vec![];
            // record
            Ok(HttpResponse::Unauthorized().finish())
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[derive(Serialize)]
pub struct EventBriefInfo {
    pub like: bool,
    pub event_id: String,
    pub event_name: String,
    pub event_time: String,
    pub sponsor_name: String,
    pub sponsor_avatar: String,
    pub start_time: String,
    pub end_time: String,
}

#[derive(Serialize)]
pub struct EnrolledEventsRet {
    pub events: Vec<EventBriefInfo>
}

#[allow(dead_code)]
pub fn get_all_enrolled_events(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match users::get_user_records(&openid) {
                Ok(records) => {
                    let mut events: Vec<EventBriefInfo> = vec![];
                    let event_list = (*EVENT_LIST).lock().unwrap();
                    for record in records {
                        let like: bool;
                        match users::check_user_like(&openid, &record.event_id) {
                            Ok(flag) => {
                                like = flag;
                            }
                            Err(_) => {
                                like = false;
                                //return Ok(HttpResponse::UnprocessableEntity().json(e));
                            }
                        }

                        let sponsor_avatar: String;
                        match sponsors::get_avatar_by_name(&record.sponsor_name) {
                            Ok(avatar_url) => {
                                sponsor_avatar = avatar_url.clone();
                            }
                            Err(e) => {
                                println!("{}", e);
                                sponsor_avatar = "http://2019-a18.iterator-traits.com/apis/sponsors\
                                /pic/default_avatar.jpg".to_string();
                            }
                        }
                        let event_name: String;
                        let event_time: String;
                        match event_list.get(&record.event_id) {
                            Some(event) => {
                                event_name = event.event_name.clone();
                                event_time = event.event_time.clone();
                            }
                            None => {
                                event_name = "".to_string();
                                event_time = "".to_string();
                            }
                        }
                            
                        events.push(EventBriefInfo {
                            like: like,
                            event_id: record.event_id,
                            event_name: event_name.clone(),
                            event_time: event_time.clone(),
                            sponsor_name: record.sponsor_name,
                            sponsor_avatar: sponsor_avatar,
                            start_time: record.start_time,
                            end_time: record.end_time
                        })
                    }
                    Ok(HttpResponse::Ok().json(EnrolledEventsRet {
                        events: events
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
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
    if let Ok(sponsor_name) = identify_user(&id) {
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

#[derive(Serialize)]
pub struct IfEnrolledRet {
    pub enrolled: bool
}

#[allow(dead_code)]
pub fn check_if_enrolled(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let event_id = req.match_info().query("event_id").to_string();
            let records = users::get_user_records(&openid).unwrap();
            let mut enrolled = false;
            for record in records {
                if (record.event_id == event_id) && (record.user_id == openid) {
                    enrolled = true;
                }
            }

            Ok(HttpResponse::Ok().json(IfEnrolledRet {
                enrolled: enrolled
            }))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
pub fn cancel_book_event(
    id: Identity,
    req: HttpRequest
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let event_id = req.match_info().query("event_id").to_string();
            match (*EVENT_LIST).try_lock() {
                Ok(a) => {println!("OKOKOK!");drop(a);}
                Err(e) => {println!("{:?}", e);}
            }
            let mut events = (*EVENT_LIST).lock().unwrap();
            match events.get_mut(&event_id) {
                Some(mut event) => {
                    if event.event_status % 10 >=2 {
                        event.left_tickets += 1;
                        event.update_type = 1;
                        let index_str = event_id + "_" + &openid;
                        let success: bool;
                        let mut records = (*RECORD).lock().unwrap();
                        success = match records.get_mut(&index_str) {
                            Some(mut record) => {
                                record.update_type = 2;
                                true
                            }
                            None => false,
                        };
                        println!("HERE!!success:{}", success);
                        drop(events);
                        drop(records);
                        update_events();
                        update_records();
                        Ok(HttpResponse::Ok().json(SuccessRet {
                            success: success
                        }))
                    } else {
                        drop(events);
                        Ok(HttpResponse::Ok().json(SuccessRet {
                            success: false
                        }))
                    }
                }
                None => {
                    drop(events);
                    Ok(HttpResponse::UnprocessableEntity().finish())
                }
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[derive(Serialize)]
pub struct MomentsRet {
    pub more: bool,
    pub moments: Vec<moment::Moment>
}

#[derive(Deserialize)]
pub struct QueryListWithSponsorName {
    pub index: usize,
    pub sponsor_name: String
}

pub fn get_sponsor_moments(
    id: Identity,
    Query(query_list): Query<QueryListWithSponsorName>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(_) => {
            match moment::get_sponsor_moments_sorted(&query_list.sponsor_name) {
                Ok(moments) => {
                    let index = query_list.index;

                    let more: bool;
                    let end: usize;
                    if index + 6 >= moments.len() {  // if no more sponsors followed
                        more = false;
                        end = moments.len();
                    } else {
                        more = true;
                        end = index + 6;
                    }
                    let mut moments_ret = vec![];
                    for i in index..end {
                        moments_ret.push(moments[i].clone());
                    }
                    Ok(HttpResponse::Ok().json(MomentsRet { // 200 OK
                        more: more,
                        moments: moments_ret,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn get_followed_sponsor_moments(
    id: Identity,
    Query(query_list): Query<QueryList>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match moment::get_user_follow_sponsor_moments_sorted(&openid) {
                Ok(moments) => {
                    let index = query_list.index;

                    let more: bool;
                    let end: usize;
                    if index + 6 >= moments.len() {  // if no more sponsors followed
                        more = false;
                        end = moments.len();
                    } else {
                        more = true;
                        end = index + 6;
                    }
                    let mut moments_ret = vec![];
                    for i in index..end {
                        moments_ret.push(moments[i].clone());
                    }
                    Ok(HttpResponse::Ok().json(MomentsRet { // 200 OK
                        more: more,
                        moments: moments_ret,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Deserialize)]
pub struct QueryListWithEventID {
    pub index: usize,
    pub event_id: String
}

pub fn get_event_moments(
    id: Identity,
    req: HttpRequest,
    Query(query_list): Query<QueryList>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(_) => {
            let event_id = req.match_info().query("event_id").to_string();
            match moment::get_event_moments_sorted(&event_id) {
                Ok(moments) => {
                    let index = query_list.index;

                    let more: bool;
                    let end: usize;
                    if index + 6 >= moments.len() {  // if no more sponsors followed
                        more = false;
                        end = moments.len();
                    } else {
                        more = true;
                        end = index + 6;
                    }
                    let mut moments_ret = vec![];
                    for i in index..end {
                        moments_ret.push(moments[i].clone());
                    }
                    Ok(HttpResponse::Ok().json(MomentsRet { // 200 OK
                        more: more,
                        moments: moments_ret,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
/*
#[allow(dead_code)]
pub fn get_random_moments(
    id: Identity,
    Query(query_list): Query<QueryList>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            // TODO
            match moment::get_random_moments_sorted(&openid) {
                Ok(moments) => {
                    let index = query_list.index;

                    let more: bool;
                    let end: usize;
                    if index + 6 >= moments.len() {  // if no more sponsors followed
                        more = false;
                        end = moments.len();
                    } else {
                        more = true;
                        end = index + 6;
                    }
                    let mut moments_ret = vec![];
                    for i in index..end {
                        moments_ret.push(moments[i].clone());
                    }
                    Ok(HttpResponse::Ok().json(MomentsRet { // 200 OK
                        more: more,
                        moments: moments_ret,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
*/
#[allow(dead_code)]
pub fn get_liked_event_moments(
    id: Identity,
    Query(query_list): Query<QueryList>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match moment::get_user_like_event_moments_ordered(&openid) {
                Ok(moments) => {
                    let index = query_list.index;

                    let more: bool;
                    let end: usize;
                    if index + 6 >= moments.len() {  // if no more sponsors followed
                        more = false;
                        end = moments.len();
                    } else {
                        more = true;
                        end = index + 6;
                    }
                    let mut moments_ret = vec![];
                    for i in index..end {
                        moments_ret.push(moments[i].clone());
                    }
                    Ok(HttpResponse::Ok().json(MomentsRet { // 200 OK
                        more: more,
                        moments: moments_ret,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
pub struct UserEventInfo {
    pub like: bool,
    pub event_id: String,
	pub sponsor_name: String,
    pub event_name: String,
    pub start_time: String,
    pub event_time: String,
    pub end_time: String,
    pub event_type: i8,
    pub event_introduction: String,
    pub event_picture: String,
    pub event_capacity: i32,
    pub current_participants: i32,
    pub left_tickets: i32,
    pub event_status: i8,
    pub event_location: String,
}

#[derive(Serialize)]
pub struct UserEventsRet {
    pub events: Vec<UserEventInfo>,
    pub more: bool
}

#[allow(dead_code)]
pub fn get_sponsor_events(
    id: Identity,
    Query(query_list): Query<QueryListWithSponsorName>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            match sponsors::get_sponsor_events(&query_list.sponsor_name) {
                Ok(events) => {
                    let index = query_list.index;

                    let more: bool;
                    let end: usize;
                    if index + 6 >= events.len() {  // if no more sponsors followed
                        more = false;
                        end = events.len();
                    } else {
                        more = true;
                        end = index + 6;
                    }
                    let mut events_ret = vec![];
                    for i in index..end {
                        let event = &events[i];
                        let like = match users::check_user_like(&openid, &event.event_id) {
                            Ok(flag) => flag,
                            Err(_) => false
                        };
                        events_ret.push(UserEventInfo {
                            like: like,
                            event_id: event.event_id.clone(),
                            sponsor_name: event.sponsor_name.clone(),
                            event_name: event.event_name.clone(),
                            start_time: event.start_time.clone(),
                            event_time: event.event_time.clone(),
                            end_time: event.end_time.clone(),
                            event_type: event.event_type,
                            event_introduction: event.event_introduction.clone(),
                            event_picture: event.event_picture.clone(),
                            event_capacity: event.event_capacity,
                            current_participants: event.current_participants,
                            left_tickets: event.left_tickets,
                            event_status: event.event_status,
                            event_location: event.event_location.clone(),
                       });
                    }
                    Ok(HttpResponse::Ok().json(UserEventsRet { // 200 OK
                        more: more,
                        events: events_ret,
                    }))
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[allow(dead_code)]
pub fn get_random_events(
    id: Identity,
    req: HttpRequest,
    Query(query_list): Query<QueryList>
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            let events = (*EVENT_LIST).lock().unwrap();
            let mut available_event_id = vec![];
            for event in events.values() {
                if (event.event_status % 10 > 0) && (event.event_status % 10 < 3) {
                    available_event_id.push(event.event_id.clone());
                }
            }
            let index = query_list.index;
            let more: bool;
            let end: usize;
            if index + 6 >= available_event_id.len() {
                    more = false;
                    end = available_event_id.len();
            } else {
                more = true;
                end = index + 6;
            }
            let mut events_ret = vec![];
            for i in index..end {
                let event = &events.get(&available_event_id[i]).unwrap();
                let like = match users::check_user_like(&openid, &event.event_id) {
                        Ok(flag) => flag,
                        Err(_) => false
                };
                events_ret.push(UserEventInfo {
                    like: like,
                    event_id: event.event_id.clone(),
                    sponsor_name: event.sponsor_name.clone(),
                    event_name: event.event_name.clone(),
                    start_time: event.start_time.clone(),
                    event_time: event.event_time.clone(),
                    end_time: event.end_time.clone(),
                    event_type: event.event_type,
                    event_introduction: event.event_introduction.clone(),
                    event_picture: event.event_picture.clone(),
                    event_capacity: event.event_capacity,
                    current_participants: event.current_participants,
                    left_tickets: event.left_tickets,
                    event_status: event.event_status,
                    event_location: event.event_location.clone(),
                });
            }
            Ok(HttpResponse::Ok().json(UserEventsRet { // 200 OK
                more: more,
                events: events_ret,
            }))
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
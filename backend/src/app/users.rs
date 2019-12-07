use std::env;
use std::fs;
use std::io::{Write};
use std::path::{PathBuf};

use actix_multipart::{Field, Multipart, MultipartError};
use actix_identity::Identity;
use actix_web::{client, error, Error, http::Method, HttpRequest, HttpResponse, ResponseError, web, web::Data, web::Json, web::Query};
use futures::{Future, future::result, Stream};
use futures::future::{err, Either};
use md5::compute;
use reqwest::{get, Client};
use serde::{Deserialize, Serialize};

use super::{APP_ID, EVENT_LIST, SECRET};
use super::sponsors::{QuerySponsor, QuerySponsorByID};
use super::events::{QueryEvent, QueryEventByID, EventsRet};
use crate::db::events;
use crate::db::sponsors;
use crate::db::users;
use crate::utils::auth::{identify_user};

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
            match users::add_user(&text.openid) {
                Ok(_) => Ok(HttpResponse::Ok().json(WechatLoginReturn {
                                openid: text.openid.clone(),
                            })),
                Err(_) => Ok(HttpResponse::Ok().json(WechatLoginReturn {
                    openid: text.openid.clone(),
                }))
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
                    users::set_tsinghua_id(&openid, &text.user.card).unwrap();
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
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            // TODO: 获得关注数目、历史数目和喜爱的数目
            match users::get_tsinghua_id(&openid) {
                Ok(tsinghua_id) => Ok(HttpResponse::NotImplemented().json(UserInfo{
                    follow: 0,
                    history: 0,
                    like: 0,
                    tsinghuaid: tsinghua_id
                })),
                Err(_) => Ok(HttpResponse::NotImplemented().json(UserInfoWithoutTHUID{
                    follow: 0,
                    history: 0,
                    like: 0,
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
    result(match id.identity() {
        Some(id) => {
            let sponsor_name = req.match_info().query("sponsor_name").to_string();
            match users::check_user_follow(&id, &sponsor_name) {
                Ok(flag) => Ok(HttpResponse::Ok().json(FollowRet { // 200 Ok
                    follow: flag,
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
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
    id: String,
    name: String,
    avatar_url: String,
}

#[derive(Serialize)]
struct FollowListRet {
    more: bool,
    list: Vec<SponsorInfo>,
}

pub fn get_follow_list(
    (id, Query(query_list)):
    (Identity, Query<QueryList>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(openid) => {
            match users::get_user_follows(&openid) {
                Ok(name_list) => {
                    let index = query_list.index;
                    if name_list.len() <= index {
                        Ok(HttpResponse::BadRequest().finish())
                    } else {
                        let mut more = true;
                        let mut list: Vec<SponsorInfo> = vec![];

                        if index + 15 >= name_list.len() {  // if no more sponsors followed
                            more = false;
                        }
                        for i in index..(index + 14) {
                            match sponsors::get_info_by_name(&name_list[i]) {
                                Ok(sponsor) => {
                                    list.push(SponsorInfo {
                                        id: sponsor.id.clone(),
                                        name: name_list[i].clone(),
                                        avatar_url: "https://image.baidu.com/search/detail?ct=503316480&z=0&ipn=d&word=影流之主&step_word=&hs=0&pn=48&spn=0&di=2670&pi=0&rn=1&tn=baiduimagedetail&is=0%2C0&istype=0&ie=utf-8&oe=utf-8&in=&cl=2&lm=-1&st=undefined&cs=562893584%2C1346842489&os=32714240%2C1997607233&simid=3364286120%2C229005232&adpicid=0&lpn=0&ln=777&fr=&fmq=1575227579031_R&fm=&ic=undefined&s=undefined&hd=undefined&latest=undefined&copyright=undefined&se=&sme=&tab=0&width=undefined&height=undefined&face=undefined&ist=&jit=&cg=&bdtype=11&oriquery=&objurl=http%3A%2F%2Fpic.fxsw.net%2Fup%2F2019-11%2F2019112310749555.jpg&fromurl=ippr_z2C%24qAzdH3FAzdH3Fooo_z%26e3Buxfo_z%26e3BgjpAzdH3Fip4sAzdH3Fd8la_z%26e3Bip4s&gsm=&rpstart=0&rpnum=0&islist=&querylist=&force=undefined".to_string().clone(),
                                    })
                                },
                                Err(_) => {
                                    break;
                                }
                            }
                        }
                        if list.len() == 15 {
                            Ok(HttpResponse::Ok().json(FollowListRet { // 200 OK
                                more: more,
                                list: list,
                            }))
                        } else {
                            Ok(HttpResponse::UnprocessableEntity().finish())
                        }
                    }
                },
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e)) // 422 Unprocessable Entity
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
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
    result(match id.identity() {
        Some(openid) => {
            let event_id = req.match_info().query("event_id").to_string();
            match users::check_user_like(&openid, &event_id) {
                Ok(flag) => Ok(HttpResponse::Ok().json(LikeRet { // 200 Ok
                    like: flag,
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
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
                    if id_list.len() <= index {
                        Ok(HttpResponse::BadRequest().finish()) // 400 Bad Request
                    } else {
                        let mut more = true;
                        let mut list: Vec<EventInfo> = vec![];

                        if index + 6 >= id_list.len() {  // if no more events liked
                            more = false;
                        }
                        for i in index..(index + 5) {
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
                                Err(e) => {
                                    break;
                                }
                            }
                        }
                        if list.len() == 6 {
                            Ok(HttpResponse::Ok().json(LikeListRet { // 200 OK
                                more: more,
                                list: list,
                            }))
                        } else {
                            Ok(HttpResponse::UnprocessableEntity().finish())
                        }
                    }
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
            match (*EVENT_LIST).lock().unwrap().get_mut(&event_id) {
                Some(mut event) => {
                    if (event.event_status == 2) && (event.left_tickets > 0) {
                        event.left_tickets -= 1;
                        event.update_type = 1;
                        // TODO:record
                        Ok(HttpResponse::Ok().json(SuccessRet {
                            success: true
                        }))
                    } else {
                        Ok(HttpResponse::Ok().json(SuccessRet {
                            success: false
                        }))
                    }
                }
                None => Ok(HttpResponse::UnprocessableEntity().finish())
            }
        }
        Err(_) => Ok(HttpResponse::Unauthorized().finish())
    })
}

#[allow(dead_code)]
pub fn get_sponsor_info(
    query_sponsor: QuerySponsor,
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(_) => {
            let sponsor: SponsorInfo;
            match sponsors::get_info_by_name(&query_sponsor.sponsor_name) {
                Ok(sponsor) => Ok(HttpResponse::Ok().json(sponsor)),
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

#[allow(dead_code)]
pub fn get_all_enrolled_events(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match identify_user(&id) {
        Ok(openid) => {
            // TODO
            Ok(HttpResponse::NotImplemented().finish())
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
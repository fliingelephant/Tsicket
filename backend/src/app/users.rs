use std::convert::From;
use actix_identity::Identity;
use actix_web::{client, Error, http::Method, HttpRequest, HttpResponse, ResponseError, web::Data, web::Json, web::Query};
use futures::{Future, future::result, future::lazy};
use reqwest::{get, Client};
use serde::{Deserialize, Serialize};
use std::env;

use super::{APP_ID, SECRET};
use super::sponsors::QuerySponsor;
use super::events::{QueryEvent, QueryEventByID};
use crate::db::events;
use crate::db::sponsors;
use crate::db::users;

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

#[derive(Deserialize)]
pub struct WechatTHUInfo {
    openid: String,
    token: String,
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
            id.remember(text.openid.to_owned());
            Ok(HttpResponse::Ok().json(WechatLoginReturn {
                openid: text.openid.clone(),
            }))
        },
        Err(e) => {
            println!("{}", e.to_string());
            Ok(HttpResponse::UnprocessableEntity().json("Wrong code"))
        }
    })
    //result(Ok(HttpResponse::Ok().json(resp.unwrap().text().unwrap())))
}

pub fn bind_tsinghua_id(
    info: Json<WechatTHUInfo>,
) -> impl Future<Item=HttpResponse, Error=Error> {
    let client = reqwest::Client::new();
    let resp = client
        .post("https://alumni-test.iterator-traits.com/fake-id-tsinghua-proxy/api/user/session/token")
        .json(&MPTHUInfo {
            token: info.token.clone(),
        })
        .send();

    let content = resp.unwrap().json::<TsinghuaInfo>();
    result(match content {
        Ok(text)=> {
            Ok(HttpResponse::Ok().json(WechatTHUReturn {
                    tsinghuaid: text.user.card.clone(),
            }))
        },
        Err(e) => {
            println!("{:?}", e.to_string());
            Ok(HttpResponse::UnprocessableEntity().json("Wrong code"))
        }
    })
}

#[derive(Serialize)]
struct UserInfo {
    follow: usize,
    history: usize,
    like: usize,
    tsinghua_id: String,
}

pub fn get_personal_info(
    id: Identity
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(openid) => {
            Ok(HttpResponse::NotImplemented().json(UserInfo{
                follow: 0,
                history: 0,
                like: 0,
                tsinghua_id: "testtsinghuaid".to_string().clone()
            }))
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
struct FollowRet {
    follow: bool,
}

pub fn check_follow(
    (id, query_sponsor):
    (Identity, Json<QuerySponsor>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(id) => {
            match users::check_user_follow(&id, &query_sponsor.sponsor_name) {
                Ok(flag) => Ok(HttpResponse::Ok().json(FollowRet { // 200 Ok
                    follow: flag,
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

pub fn follow_or_unfo(
    (id, query_sponsor):
    (Identity, Json<QuerySponsor>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(id) => {
            // TODO
            Ok(HttpResponse::NotImplemented().json(FollowRet {
                follow: true,
            }))
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
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
    (id, query_list):
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
                            match sponsors::get_info_by_name(&name_list[index]) {
                                Ok(sponsor) => {
                                    list.push(SponsorInfo {
                                        id: sponsor.id.clone(),
                                        name: name_list[index].clone(),
                                        avatar_url: "https://image.baidu.com/search/detail?ct=503316480&z=0&ipn=d&word=影流之主&step_word=&hs=0&pn=48&spn=0&di=2670&pi=0&rn=1&tn=baiduimagedetail&is=0%2C0&istype=0&ie=utf-8&oe=utf-8&in=&cl=2&lm=-1&st=undefined&cs=562893584%2C1346842489&os=32714240%2C1997607233&simid=3364286120%2C229005232&adpicid=0&lpn=0&ln=777&fr=&fmq=1575227579031_R&fm=&ic=undefined&s=undefined&hd=undefined&latest=undefined&copyright=undefined&se=&sme=&tab=0&width=undefined&height=undefined&face=undefined&ist=&jit=&cg=&bdtype=11&oriquery=&objurl=http%3A%2F%2Fpic.fxsw.net%2Fup%2F2019-11%2F2019112310749555.jpg&fromurl=ippr_z2C%24qAzdH3FAzdH3Fooo_z%26e3Buxfo_z%26e3BgjpAzdH3Fip4sAzdH3Fd8la_z%26e3Bip4s&gsm=&rpstart=0&rpnum=0&islist=&querylist=&force=undefined".to_string().clone(),
                                    })
                                },
                                Err(e) => {
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
    (id, query_event):
    (Identity, Json<QueryEventByID>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(openid) => {
            match users::check_user_like(&openid, &query_event.event_id) {
                Ok(flag) => Ok(HttpResponse::Ok().json(LikeRet { // 200 Ok
                    like: flag,
                })),
                Err(e) => Ok(HttpResponse::UnprocessableEntity().json(e))
            }
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

pub fn like_or_dislike(
    (id, query_event):
    (Identity, Json<QueryEventByID>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(id) => {
            // TODO
            Ok(HttpResponse::NotImplemented().json(LikeRet {
                like: true,
            }))
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
struct EventInfo {
    event_id: String,
    event_name: String,
    img_url: String,
    intro: String,
    start_time: String,
}

#[derive(Serialize)]
struct LikeListRet {
    more: bool,
    list: Vec<EventInfo>,
}

pub fn get_like_list(
    (id, query_list):
    (Identity, Query<QueryList>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(openid) => {
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
                            match events::get_info_by_id(&id_list[index]) {
                                Ok(event) => {
                                    list.push(EventInfo {
                                        event_id: id_list[index].clone(),
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
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
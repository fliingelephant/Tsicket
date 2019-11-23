use std::convert::From;
use actix_identity::Identity;
use actix_web::{client, Error, http::Method, HttpRequest, HttpResponse, ResponseError, web::Data, web::Json, web::Query};
use futures::{Future, future::result, future::lazy};
use reqwest::{get, Client};
use serde::{Deserialize, Serialize};
use std::env;

use super::{APP_ID, SECRET};
use super::sponsors::QuerySponsor;
use super::events::QueryEvent;

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

#[derive(Debug, Deserialize)]
pub struct RegisterUser {
    pub username: String,
    pub password: String,
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
struct FollowFlag {
    follow: bool,
}

pub fn check_follow(
    (id, query_sponsor):
    (Identity, Json<QuerySponsor>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(id) => {
            // TODO
            Ok(HttpResponse::NotImplemented().json(FollowFlag{ // 501 Not Implemented
                follow: false,
            }))
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}

#[derive(Serialize)]
struct LikeFlag {
    like: bool,
}

pub fn check_like(
    (id, query_event):
    (Identity, Json<QueryEvent>)
) -> impl Future<Item=HttpResponse, Error=Error> {
    result(match id.identity() {
        Some(id) => {
            // TODO
            Ok(HttpResponse::NotImplemented().json(LikeFlag{ // 501 Not Implemented
                like: false,
            }))
        },
        None => Ok(HttpResponse::Unauthorized().finish()) // 401 Unauthorized
    })
}
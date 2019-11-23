extern crate mysql;

use std::env;

use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web::{
    App,
    // http::header::{AUTHORIZATION, CONTENT_TYPE},
    HttpRequest,
    HttpServer,
    middleware,
    web,
    //web::Data,
};
use mysql as my;
//use rand::FromEntropy;
use rand::prelude::*;

use crate::db;
use crate::db::events::Event;
use crate::db::records::Record;


mod admins;
mod events;
mod sponsors;
mod users;
mod update;
mod init;

lazy_static! {
    pub static ref ADMIN_ID: String
        = env::var("ADMIN_ID").expect("Admin's id must be set!");
    pub static ref ADMIN_PASSWORD_WITH_SALT: String
        = env::var("ADMIN_PASSWORD_WITH_SALT").expect("Admin's password must be set!");
    pub static ref DATABASE_URL: String
        = env::var("DATABASE_URL").expect("Database URL must be set!");
    pub static ref EVENTS_TO_CHECK: Vec<db::events::Event> = Vec::new();
    pub static ref POOL:my::Pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
}

fn index(id: Identity) -> String {
    println!("{:?}", id.identity());
    if let Some(id) = id.identity() {
        format!("Hello {}!", id)
    } else {
        "Hello World!".to_string()
    }
}

pub async fn update() -> () {}

pub fn start() -> () {
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    /*let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    let mut cookie_private_key = [0u8; 32];
    let mut rng = rand::StdRng::from_entropy();
    rng.fill(&mut cookie_private_key[..]);

    HttpServer::new(move || {
        App::new()
            .configure(routes)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_private_key[..])
                    .name("auth-cookie")
                    .secure(false),
            ))
    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .start();*/
    /*let mut events:Vec<Event> = Vec::new();
    let mut records:Vec<Record> = Vec::new();
    let dd = init::initiate(events.as_mut(), records.as_mut());
    events[0].event_name = "!!!".to_string();
    events[0].update_type = 1;
    let re  = Record{
        record_id: "0000".to_string(),
        event_id: "999".to_string(),
        sponsor_name: "zjr".to_string(),
        user_id: "cba".to_string(),
        start_time: "1970-02-26 20:47:46".to_string(),
        end_time: "2019-11-18 20:47:46".to_string(),
        update_type: 3
    };
    records.push(re);
    update::update(events.as_mut(), records.as_mut());*///测试更新函数
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/").to(index))
        .service(web::scope("/apis")
                     /* User routes ↓ */
                     /* TODO
                     .service(web::resource("users")
                         .route(web::get().to_async(users::get_personal_data))
                         .route(web::post().to_async(users::get_enrolled_events))
                     ) */
                     .service(web::resource("users/register")
                         .route(web::post().to_async(users::register))
                     )
                     .service(web::resource("users/login")
                         .route(web::post().to_async(users::login))
                     )

                     /* Sponsor routes ↓ */
                     /* TODO
                     .service(web::resource("sponsors")
                         .route(web::post().to_async(publishers::publish_event))
                     ) */
                     .service(web::resource("sponsors/register")
                         .route(web::post().to_async(sponsors::register))
                     )
                     .service(web::resource("sponsors/login")
                         .route(web::post().to_async(sponsors::login))
                     )
                     .service(web::resource("sponsors/logout")
                         .route(web::post().to_async(sponsors::logout))
                     )

                     /* Administrator routes ↓ */
                     .service(web::resource("admins")
                         .route(web::get().to_async(events::get_all_events))
                     )
                     .service(web::resource("admins/login")
                         .route(web::post().to_async(admins::login))
                     )
                     .service(web::resource("admins/logout")
                         .route(web::post().to_async(admins::logout))
                     )

                 /* Events routes */
                 /* TODO
                 .service(web::resource("events")
                     .route(web::post().to_async(events::bookEvent))
                 )
                 .service(web::resource("events/view")
                     .route(web::get().to_async(events::eventInfo))
                 ) */
        );
}

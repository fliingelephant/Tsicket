extern crate mysql;


use std::env;
use std::sync::{Mutex};

use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web::{
    App,
    // http::header::{AUTHORIZATION, CONTENT_TYPE},
    HttpRequest,
    HttpServer,
    middleware,
    web,
    web::Data,
};
use mysql as my;
use rand::{thread_rng, Rng};

use crate::db;

mod admins;
mod events;
mod init;
mod sponsors;
mod users;
mod update;

lazy_static! {
    pub static ref ADMIN_ID: String
        = env::var("ADMIN_ID").expect("Admin's id must be set!");
    pub static ref ADMIN_PASSWORD_WITH_SALT: String
        = env::var("ADMIN_PASSWORD_WITH_SALT").expect("Admin's password must be set!");
    pub static ref APP_ID: String
        = env::var("APP_ID").expect("App id must be set!");
    pub static ref DATABASE_URL: String
        = env::var("DATABASE_URL").expect("Database URL must be set!");
    pub static ref EVENTS_TO_CHECK: Vec<db::events::Event> = Vec::new();
    pub static ref POOL:my::Pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    pub static ref SECRET: String
        = env::var("SECRET").expect("Wechat secret must be set!");
}

pub struct EventState {
    pub event_list: Vec<db::events::Event>,
    //pub record: 
}

fn index(id: Identity) -> String {
    println!("{:?}", id.identity());
    if let Some(id) = id.identity() {
        format!("Hello {}!", id)
    } else {
        "Hello World!".to_string()
    }
}

pub fn start() -> () {
    //let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    let mut cookie_private_key = [0u8; 32];
    thread_rng().fill(&mut cookie_private_key[..]);

    HttpServer::new(move || {
        App::new()
            .register_data(Data::new(Mutex::new(EventState {
                event_list: Vec::new(),
            })))
            .configure(routes)
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookie_private_key[..])
                    .name("auth-cookie")
                    .secure(false),
            ))
    })
        .bind(&bind_address)
        .unwrap_or_else(|_| panic!("Could not bind server to address {}", &bind_address))
        .start();
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
                    .service(web::resource("users/book")
                        .route(web::post().to_async(events::book_event))
                    )
                    .service(web::resource("users/broadcast")
                        .route(web::get().to_async(events::get_broadcast_events))
                    )
                    .service(web::resource("users/follow")
                        .route(web::post().to_async(users::check_follow))
                    )
                    .service(web::resource("users/like")
                        .route(web::post().to_async(users::check_like))
                    )
                    .service(web::resource("users/login")
                        .route(web::post().to_async(users::login))
                    )
                     .service(web::resource("users/tsinghuaid")
                        .route(web::post().to_async(users::bind_tsinghua_id))
                    )

                     /* Sponsor routes ↓ */
                     .service(web::resource("sponsors")
                         .route(web::get().to_async(sponsors::get_events))
                         .route(web::post().to_async(sponsors::publish_event))
                     )
                     .service(web::resource("sponsors/register")
                         .route(web::post().to_async(sponsors::register))
                     )
                     .service(web::resource("sponsors/login")
                         .route(web::post().to_async(sponsors::login))
                     )
                     .service(web::resource("sponsors/logout")
                         .route(web::post().to_async(sponsors::logout))
                     )
                     .service(web::resource("sponsors/view")
                         .route(web::get().to_async(sponsors::get_sponsor_info))
                         .route(web::put().to_async(sponsors::alter_sponsor_info))
                     )

                     /* Administrator routes ↓ */
                     .service(web::resource("admins")
                         .route(web::get().to_async(admins::get_all_events))
                     )
                     .service(web::resource("admins/login")
                         .route(web::post().to_async(admins::login))
                     )
                     .service(web::resource("admins/logout")
                         .route(web::post().to_async(admins::logout))
                     )

                     
                     .service(web::resource("events/view")
                         .route(web::get().to_async(events::get_event_info))
                     )

                 /* Events routes */
                 /*
                 .service(web::resource("events")
                     .route(web::post().to_async(events::bookEvent))
                 )*/
        );
}
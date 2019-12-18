use std::collections::{HashMap};
use std::env;
use std::sync::{Mutex};

use actix_files;
use actix_identity::{Identity, CookieIdentityPolicy, IdentityService};
use actix_web::{
    App,
    HttpResponse,
    HttpServer,
    middleware,
    web,
    web::Data,
};
use mysql::{Pool};
use rand::{thread_rng, Rng};

use crate::db;
use crate::db::events::Event;
use crate::db::records::{Record};
use init::initiate;

pub mod admins;
mod events;
mod init;
mod sponsors;
mod users;
mod update;

/* Configuration String */
lazy_static! {
    pub static ref ADMIN_ID: String
        = env::var("ADMIN_ID").expect("Admin's id must be set!");
    pub static ref ADMIN_PASSWORD_WITH_SALT: String
        = env::var("ADMIN_PASSWORD_WITH_SALT").expect("Admin's password must be set!");
    pub static ref APP_ID: String
        = env::var("APP_ID").expect("App id must be set!");
    pub static ref DATABASE_URL: String
        = env::var("DATABASE_URL").expect("Database URL must be set!");
    pub static ref SECRET: String
        = env::var("SECRET").expect("Wechat secret must be set!");
}

/* Static Data */
lazy_static! {
    pub static ref BROADCAST_LIST: Mutex<Vec<String>> = Mutex::new(Vec::new());
    pub static ref EVENT_LIST: Mutex<HashMap<String, Event>> = Mutex::new(HashMap::new());
    pub static ref RECORD: Mutex<HashMap<String, Record>> = Mutex::new(HashMap::new());
    pub static ref POOL: Pool = Pool::new(&*DATABASE_URL).unwrap();
    //pub static ref RECORD: Mutex<Vec<Record>> = Mutex::bew(Ha)
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title></head>
        <body>
            <form target="/apis/sponsors/upload" method="post" enctype="multipart/form-data">
                <input type="file" name="file"/>
                <input type="submit" value="Submit"></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

pub fn start() -> () {
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS is not set");

    let mut cookie_private_key = [0u8; 32];
    thread_rng().fill(&mut cookie_private_key[..]);

    initiate(& mut *EVENT_LIST.lock().unwrap(), & mut * RECORD.lock().unwrap()).unwrap();

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
        .start();
}

fn routes(app: &mut web::ServiceConfig) {
    app
        .service(web::resource("/").to(index))
        .service(web::scope("/apis")
                     /* User routes ↓ */
                    
                    .service(web::resource("users/view")
                        .route(web::get().to_async(users::get_personal_info))
                    )
                    .service(web::resource("users/book")
                        .route(web::get().to_async(users::get_all_enrolled_events))
                    )
                    .service(web::resource("users/book/{event_id}")
                        .route(web::get().to_async(users::check_if_enrolled))
                        .route(web::post().to_async(users::book_event))
                        .route(web::delete().to_async(users::cancel_book_event))
                    )
                    .service(web::resource("users/broadcast")
                        .route(web::get().to_async(events::get_broadcast_events))
                    )
                    .service(web::resource("users/events")
                        .route(web::get().to_async(users::get_sponsor_events))
                    )
                    .service(web::resource("users/follow")
                        .route(web::get().to_async(users::get_follow_list))
                    )
                    .service(web::resource("users/follow/{sponsor_name}")
                        .route(web::get().to_async(users::check_follow))
                        .route(web::post().to_async(users::follow_or_unfo))
                    )
                    .service(web::resource("users/index")
                        .route(web::get().to_async(users::get_random_events))
                    )
                    .service(web::resource("users/like")
                        .route(web::get().to_async(users::get_like_list))
                    )
                    .service(web::resource("users/like/{event_id}")
                        .route(web::get().to_async(users::check_like))
                        .route(web::post().to_async(users::like_or_dislike))
                    )
                    .service(web::resource("users/login")
                        .route(web::post().to_async(users::login))
                    )
                    .service(web::resource("users/moments")
                        .route(web::get().to_async(users::get_sponsor_moments))
                    )
                    .service(web::resource("users/moments/{event_id}")
                        .route(web::get().to_async(users::get_event_moments))
                    )
                    .service(web::resource("users/momentsfollow")
                        .route(web::get().to_async(users::get_followed_sponsor_moments))
                    )
                    .service(web::resource("users/momentslike")
                        .route(web::get().to_async(users::get_liked_event_moments))
                    )
                    //.service(web::resource("users/moments/random")
                    //    .route(web::get().to_async(users::get_random_moments))
                    //)
                    .service(web::resource("users/pic/{filename}")
                         .route(web::get().to_async(users::get_pic))
                         .route(web::post().to_async(users::update_pic))
                     )
                     .service(web::resource("users/sponsors/{sponsor_name}")
                         .route(web::get().to_async(users::get_sponsor_info))
                     )
                     .service(web::resource("users/tsinghuaid")
                        .route(web::get().to_async(users::get_tsinghua_id))
                        .route(web::post().to_async(users::bind_tsinghua_id))
                     )
                    
                    
                     /* Sponsor routes ↓ */
                     .service(web::resource("sponsors")
                         .route(web::get().to_async(sponsors::get_available_events))
                         .route(web::post().to_async(sponsors::publish_event))
                     )
                     .service(web::resource("sponsors/advertise/{event_id}")
                         .route(web::post().to_async(sponsors::advertise_event))
                         .route(web::delete().to_async(sponsors::cancel_advertise_event))
                     )
                     .service(web::resource("sponsors/book/{event_id}")
                         .route(web::delete().to_async(sponsors::cancel_event))
                     )
                     .service(web::resource("sponsors/register")
                         .route(web::post().to_async(sponsors::register))
                     )
                     /*.service(web::resource("sponsors/review")
                         .route(web::post().to_async(sponsors::review_event))
                     )*/
                     .service(web::resource("sponsors/login")
                         .route(web::post().to_async(sponsors::login))
                     )
                     .service(web::resource("sponsors/logout")
                         .route(web::post().to_async(sponsors::logout))
                     )
                     .service(web::resource("sponsors/pic/{filename}")
                         .route(web::get().to_async(sponsors::get_pic))
                         .route(web::post().to_async(sponsors::update_pic))
                     )
                     .service(web::resource("sponsors/view")
                         .route(web::get().to_async(sponsors::get_sponsor_info))
                         .route(web::put().to_async(sponsors::alter_sponsor_info))
                     )
                     .service(web::resource("sponsors/view/{sponsor_name}")
                        .route(web::get().to_async(users::get_sponsor_info))
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
                     .service(web::resource("admins/advertise")
                         .route(web::get().to_async(admins::get_ad_applications))
                     )
                     .service(web::resource("admins/advertise/{event_id}")
                         .route(web::post().to_async(admins::allow_advertise))
                         .route(web::delete().to_async(admins::cancel_advertise))
                     )
                     .service(web::resource("admins/review/{event_id}")
                         .route(web::post().to_async(admins::review_event))
                         .route(web::delete().to_async(admins::cancel_review_event))
                     )
                     .service(web::resource("admins/sponsors")
                         .route(web::get().to_async(admins::get_all_sponsor_info))
                     )
                     .service(web::resource("admins/sponsors/{sponsor_name}")
                         .route(web::get().to_async(admins::get_sponsor_info))
                     )
                     .service(web::resource("admins/cancel/{event_id}")
                         .route(web::put().to_async(admins::cancel_event))
                     )
                     
                     
                     .service(web::resource("events/view")
                         .route(web::post().to_async(events::get_event_info))
                         .route(web::put().to_async(events::alter_event_info))
                     )
                     .service(web::resource("events/moments")
                         .route(web::get().to_async(events::get_sponsor_moments))
                     )
                     .service(web::resource("events/moments/{event_id}")
                         .route(web::get().to_async(events::get_event_moments))
                         .route(web::post().to_async(sponsors::publish_moment))
                     )
                     .service(web::resource("events/posts/{event_id}")
                         .route(web::get().to_async(events::get_event_posts))
                         .route(web::post().to_async(sponsors::publish_post))
                     )
        );
}
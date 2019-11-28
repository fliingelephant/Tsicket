#[macro_use]
extern crate lazy_static;
extern crate serde_derive;
extern crate serde_json;

use crate::db::records::Record;

mod app;
mod db;
mod utils;

fn main() {
    /*dotenv::dotenv().ok();

    let sys = actix::System::new("conduit");

    app::start();

    let _ = sys.run();*/

    /*println!("result of user sign up: {}", user_sign_up("999".to_string(), "zjr".to_string()));

    println!("result of sponsor sign up: {}", sponsor_sign_up("123".to_string(), "1".to_string(), "123456".to_string()));
    let events = get_sponsor_events("1".to_string());
    println!("{}", events[0].event_introduction); //还没写添加活动

    println!("result of sponsor successful log in : {}", sponsor_log_in("123".to_string(), "123456".to_string()));
    println!("result sponsor log in with wrong password: {}", sponsor_log_in("123".to_string(), "000000".to_string()));
    println!("result sponsor log in with invalid id: {}", sponsor_log_in("0".to_string(), "123456".to_string()));

    println!("result of admin sign up: {}", admin_sign_up("0011".to_string(), "yzj".to_string(), "123".to_string()));
    println!("result of admin successful log in : {}", admin_log_in("0011".to_string(), "123".to_string()));
    println!("result admin log in with wrong password: {}", admin_log_in("0011".to_string(), "321".to_string()));
    println!("result admin log in with invalid id: {}", admin_log_in("0".to_string(), "123".to_string()));
    let mut events = get_sponsor_events("1".to_string());
    println!("{}", events.len());
    println!("{}", events[0].event_introduction);
    events[0].event_introduction = "123".to_string();
    events[0].start_time = "1988-09-10 23:11:28".to_string();
    events[0].update_type = 1;
    update(events);*/
    //let mut records = db::users::get_user_records(&"123".to_string()).ok().unwrap();
    //println!("{}", records.len());
    //println!("{}", records[0].event_id);
    //db::sponsors::sponsor_register(&"001".to_string(), &"zhh".to_string(), &"123".to_string());
    /*let re = db::users::check_user_by_id(&"123".to_string());
    match re{
        Err(e)=> println!("{}", e.to_string()),
        Ok(o)=> println!("{}", o.to_string()),
    }*/
    //let s = db::sponsors::get_info_by_name(&"2".to_string());
    //println!("{}", s.ok().unwrap().head_portrait);
    /*let mut sp = s.ok().unwrap();
    sp.email="123".to_string();
    let ss = db::sponsors::alter_sponsor_info(&sp);
    match ss{
        Err(e)=>println!("{}", e.to_string()),
        Ok(o)=>{},
    }*/
    /*let s = db::users::get_user_follows(&"123".to_string());
    match s{
        Err(e)=>println!("{}", e.to_string()),
        Ok(o)=>{for i in o{
            println!("{}", i)
        }},
    }*/
    /*let s = db::users::check_user_follow(&"13".to_string(), &"7".to_string());
    match s{
        Err(e)=>println!("{}", e.to_string()),
        Ok(o)=>{println!("{}", o)},
    }*/
    /*let s = db::sponsors::get_sponsor_events(&"2".to_string());
    match s{
        Err(e) => println!("{}", e),
        Ok(el) => {
            for i in el{
                println!("{:?}", i);
            }
        }
    }*/
    /*let pic1 = "123".to_string();
    let pic2 = "456".to_string();
    let pic = vec![pic1, pic2];
    let s = db::sponsors::publish_moment(&"2".to_string(), &"0".to_string(), &"jjj".to_string(), &pic);
    match s{
        Err(e) => println!("{}", e),
        Ok(o) => {},
    }*/
    let s = db::sponsors::release_push(&"2".to_string(), &"0".to_string(), &"活动延期".to_string());
    match s{
        Err(e) => println!("{}", e),
        Ok(o) => {},
    }
}

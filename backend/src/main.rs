#[macro_use]
extern crate lazy_static;
extern crate serde_derive;
extern crate serde_json;

mod app;
mod db;
mod utils;

fn main() {

    dotenv::dotenv().ok();

    let sys = actix::System::new("conduit");

    app::start();

    let _ = sys.run();
    
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
    /*let s = db::sponsors::get_info_by_name(&"2".to_string());
    //println!("{}", s.ok().unwrap().head_portrait);
    let mut sp = s.ok().unwrap();
    sp.head_portrait="dafadsfad".to_string();
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
    let s = db::moment::publish_moment(&"xx学生会".to_string(), &"6752fd533376ab9c8c408a697cf56566".to_string(), &"活动2".to_string(), &"11".to_string(), &"tesxty".to_string(), &pic);
    match s{
        Err(e) => println!("{}", e),
        Ok(o) => {},
    }*/
    /*let s = db::sponsors::release_push(&"2".to_string(), &"0".to_string(), &"活动延期".to_string());
    match s{
        Err(e) => println!("{}", e),
        Ok(o) => {},
    }*/
    /*let s = db::users::check_tsinghua_id(&"132".to_string());
    match s{
        Err(e) => println!("{}", e),
        Ok(o) => println!("{}", o),
    }*/
    /*let s = db::users::cancel_user_follow(&"123".to_string(), &"7".to_string());
    match s{
        Err(e) => println!("{}", e),
        Ok(_) => {},
    }*/

//    let s = db::moment::get_event_moments_sorted(&"0".to_string());
//    match s{
//        Err(e) => println!("{}", e),
//        Ok(o) => {
//            println!("{}", o.len());
//            for i in 0..o.len(){
//                println!("{}", o[i].text);
//                for j in 0..o[i].pictures.len(){
//                    println!("{}", o[i].pictures[j]);
//                }
//                println!("{}", o[i].time);
//            }
//        },
//    }
//    let s = db::users::get_user_follow_number(&"23".to_string());
//    match s{
//        Err(e) => println!("{}", e),
//        Ok(o) => {println!("{}", o)},
//    }

    /*let s = db::moment::get_user_follow_sponsor_moments_sorted(&"123".to_string());
    match s{
        Err(e) => println!("{}", e),
        Ok(o) => {
            for moment in o{
                println!("sponsor_name:{}, event_id:{}, moment_id:{}, text:{}, time:{}",
                         moment.sponsor_name, moment.event_id, moment.moment_id, moment.text, moment.time);
            }
        },
    }*/
//    let s = db::moment::get_user_like_event_moments_ordered(&"oXkuv4ne9bcnE1aoVQvHkLNMMQy4".to_string());
//    match s{
//        Err(e) => println!("{}", e),
//        Ok(o) => {
//            for moment in o{
//                println!("sponsor_name:{}, event_id:{}, moment_id:{}, text:{}, time:{}",
//                         moment.sponsor_name, moment.event_id, moment.moment_id, moment.text, moment.time);
//            }
//        },
//    }
//    let s = db::sponsors::get_all_sponsor_info();
//    match s {
//        Err(e) => println!("{}", e),
//        Ok(o) => {
//            for sponsor in o{
//                println!("id:{}, name:{}, head:{}, email:{}, number:{}",
//                         sponsor.id, sponsor.sponsor_name, sponsor.head_portrait, sponsor.email, sponsor.phone_number);
//            }
//        }
//    }
}

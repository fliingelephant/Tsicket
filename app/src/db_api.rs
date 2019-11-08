extern crate mysql;

use mysql as my;
use std::*;

struct Event{
    event_id: String,
    publisher_id: String,
    publisher_name: String,
    event_name: String,
    start_time: String,
    end_time: String,
    event_type: i8,
    event_introduction: String,
    event_capacity: i32,
    current_participants: i32,
    left_tickets: i32,
    event_status: i8,
    event_location: String,
}

fn format_string(mut src: String)->String{
    let len = src.len();
    let rlt = src[1..len-1].to_string();
    return rlt;
}

fn user_sign_up(id: &str, name: &str, password: &str)->bool{
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let mut command = "INSERT INTO admin_account (account_id, username, password) VALUES (".to_string();
    command = command + "'" + id + "',";
    command = command + "'" + name + "',";
    command = command + "'" + password + "');";
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return true,
        Result::Err(_err)=> return false,
    }
}

fn get_publisher_events(id: &str)->Vec<Event>{
    let mut event_list: Vec<Event> = vec![];

    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let mut command = "SELECT * FROM event WHERE publisher_id=".to_string();
    command = command + "'" + id + "';";
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
    match req {
        Result::Err(_err)=> return event_list,
        _ => {}
    }
    for row in req.unwrap(){
        let ev = row.unwrap().unwrap();
        let event = Event{
            event_id: format_string(ev[0].as_sql(true)),
            publisher_id: format_string(ev[1].as_sql(true)),
            publisher_name: format_string(ev[2].as_sql(true)),
            event_name: format_string(ev[3].as_sql(true)),
            start_time: format_string(ev[4].as_sql(true)),
            end_time: format_string(ev[5].as_sql(true)),
            event_type: ev[6].as_sql(true).parse().unwrap(),
            event_introduction: format_string(ev[7].as_sql(true)),
            event_capacity: ev[8].as_sql(true).parse().unwrap(),
            current_participants: ev[9].as_sql(true).parse().unwrap(),
            left_tickets: ev[10].as_sql(true).parse().unwrap(),
            event_status: ev[11].as_sql(true).parse().unwrap(),
            event_location: format_string(ev[12].as_sql(true))
        };
        event_list.push(event);
    }
    return event_list;
}

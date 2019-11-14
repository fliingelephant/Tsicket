extern crate mysql;
extern crate md5;
extern crate lazy_static;
extern crate time;

use mysql as my;
use std::*;

lazy_static::lazy_static!{
    static ref POOL:my::Pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
}

struct Event{
    event_id: String,
    sponsor_name: String,
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
    update_type: i8,
}

fn format_string(mut src: String)->String{
    let len = src.len();
    src = src[1..len-1].to_string();
    return src;
}

fn str_to_timestamp(str: String)->i64{
    let tm = time::strptime(&str, "%Y-%m-%d %H:%M:%S");
    return tm.unwrap().to_timespec().sec;
}

fn timestamp_to_str(timestamp: i64)->String{
    //待补充
    return "1".to_string();
}

fn user_sign_up(id: String, name: String)->bool{
    let command = format!("INSERT INTO user_account (account_id, nickname) VALUES\
     ('{id}', '{name}');", id=id, name=name);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return true,
        Result::Err(_err)=> return false,
    }
}

fn sponsor_sign_up(id: String, name: String, raw_password: String)->String{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("INSERT INTO sponsor_account (account_id, sponsor_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id=id, name=name, password=password);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return "success".to_string(),
        Result::Err(_err)=> return _err.to_string(),
    }
}

fn sponsor_log_in(id :String, raw_password: String)->i8{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("SELECT password FROM sponsor_account WHERE account_id='{id}';", id=id);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Err(_err) =>return -1,
        _ => {},
    }
    for row in req.unwrap(){
        let pwd = format_string(row.unwrap().unwrap()[0].as_sql(true));
        if password == pwd{
            return 1;
        } else {
            return 0;
        }
    }
    return -1;
}//返回值：-1：账号不存在，0：密码错误， 1：登录成功

fn admin_sign_up(id: String, name: String, raw_password: String)->String{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("INSERT INTO admin_account (account_id, admin_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id=id, name=name, password=password);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return "success".to_string(),
        Result::Err(_err)=> return _err.to_string(),
    }
}

fn admin_log_in(id :String, raw_password: String)->i8{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("SELECT password FROM admin_account WHERE account_id='{id}';", id=id);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Err(_err) => return -1,
        _ => {}
    }
    for row in req.unwrap(){
        let pwd = format_string(row.unwrap().unwrap()[0].as_sql(true));
        if password == pwd{
            return 1;
        } else {
            return 0;
        }
    }
    return -1;
}//返回值：-1：账号不存在，0：密码错误， 1：登录成功

fn get_sponsor_events(name: String)->Vec<Event>{
    let mut event_list: Vec<Event> = vec![];
    let command = format!("SELECT * FROM event WHERE sponsor_name='{name}';", name=name);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Err(_err)=> return event_list,
        _ => {}
    }
    for row in req.unwrap(){
        let ev = row.unwrap().unwrap();
        let event = Event{
            event_id: format_string(ev[0].as_sql(true)),
            sponsor_name: format_string(ev[1].as_sql(true)),
            event_name: format_string(ev[2].as_sql(true)),
            start_time: format_string(ev[3].as_sql(true)),
            end_time: format_string(ev[4].as_sql(true)),
            event_type: ev[5].as_sql(true).parse().unwrap(),
            event_introduction: format_string(ev[6].as_sql(true)),
            event_capacity: ev[7].as_sql(true).parse().unwrap(),
            current_participants: ev[8].as_sql(true).parse().unwrap(),
            left_tickets: ev[9].as_sql(true).parse().unwrap(),
            event_status: ev[10].as_sql(true).parse().unwrap(),
            event_location: format_string(ev[11].as_sql(true)),
            update_type: 0
        };
        event_list.push(event);
    }
    return event_list;
}

fn update(events: Vec<Event>){
    for event in events{
        if event.update_type == 1 { //修改
            let command = format!("UPDATE event SET sponsor_name='{sponsor_name}', event_name='{event_name}', \
                                    start_time='{start_time}', end_time='{end_time}', event_type={event_type}, \
                                    event_introduction='{event_introduction}', event_capacity={event_capacity}, \
                                    current_participants={current_participants}, left_tickets={left_tickets}, \
                                    event_status={event_status}, event_location='{event_location}' \
                                    WHERE event_id='{event_id}';", sponsor_name=event.sponsor_name,
                                  event_name=event.event_name, start_time=event.start_time,
                                  end_time=event.end_time, event_type=event.event_type,
                                  event_introduction=event.event_introduction, event_capacity=event.event_capacity,
                                  current_participants=event.current_participants, left_tickets=event.left_tickets,
                                  event_status=event.event_status, event_location=event.event_location,
                                  event_id=event.event_id);
            println!("{}", command);
            let req=POOL.prep_exec(command, ());
            match req {
                Result::Err(_err) => println!("{}", _err.to_string()),
                _ => {}
            }
        }
        else if event.update_type == 2 { //添加
            let command = format!("INSERT INTO event (event_id, sponsor_name, event_name, start_time, end_time, \
                                    event_type, event_introduction, event_capacity, current_participants, \
                                    left_tickets, event_status, event_location) VAlUES ('{event_id}', \
                                    '{sponsor_name}', '{event_name}', '{start_time}', '{end_time}', {event_type}, \
                                    '{event_introduction}', {event_capacity}, {current_participants}, \
                                    {left_tickets}, {event_status}, '{event_location}');", event_id=event.event_id,
                                  sponsor_name=event.sponsor_name, event_name=event.event_name,
                                  start_time=str_to_timestamp(event.start_time),
                                  end_time=str_to_timestamp(event.end_time), event_type=event.event_type,
                                  event_introduction=event.event_introduction, event_capacity=event.event_capacity,
                                  current_participants=event.current_participants, left_tickets=event.left_tickets,
                                  event_status=event.event_status, event_location=event.event_location);
            let req = POOL.prep_exec(command, ());
            match req {
                Result::Err(_err) => println!("{}", _err.to_string()),
                _ => {}
            }
        }
    }
}
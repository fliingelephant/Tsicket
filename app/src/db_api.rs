extern crate mysql;
extern crate md5;

use mysql as my;
use std::*;

struct Event{
    event_id: String,
    sponsor_id: String,
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
}

fn format_string(mut src: String)->String{
    let len = src.len();
    let rlt = src[1..len-1].to_string();
    return rlt;
}

fn user_sign_up(id: &str, name: &str)->bool{
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("INSERT INTO user_account (account_id, nickname) VALUES\
     ('{id}', '{name}');", id=id, name=name);
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return true,
        Result::Err(_err)=> return false,
    }
}

fn sponsor_sign_up(id: &str, name: &str, raw_password: &str)->bool{
    let password = format!("{:x}", md5::compute(raw_password));
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("INSERT INTO sponsor_account (account_id, sponsor_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id=id, name=name, password=password);
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return true,
        Result::Err(_err)=> return false,
    }
}

fn sponsor_log_in(id :&str, raw_password: &str)->i8{
    let password = format!("{:x}", md5::compute(raw_password));
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("SELECT password FROM sponsor_account WHERE account_id='{id}';", id=id);
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
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

fn admin_sign_up(id: &str, name: &str, raw_password: &str)->bool{
    let password = format!("{:x}", md5::compute(raw_password));
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("INSERT INTO admin_account (account_id, admin_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id=id, name=name, password=password);
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return true,
        Result::Err(_err)=> return false,
    }
}

fn admin_log_in(id :&str, raw_password: &str)->i8{
    let password = format!("{:x}", md5::compute(raw_password));
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("SELECT password FROM admin_account WHERE account_id='{id}';", id=id);
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
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

fn get_sponsor_events(name: &str)->Vec<Event>{
    let mut event_list: Vec<Event> = vec![];
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("SELECT * FROM event WHERE sponsor_name='{name}'", name=name);
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
            sponsor_id: format_string(ev[1].as_sql(true)),
            sponsor_name: format_string(ev[2].as_sql(true)),
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

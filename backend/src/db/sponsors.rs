extern crate md5;
extern crate mysql;

use std::*;
use std::result::Result;

use serde::{Serialize};

use mysql as my;

use super::events::Event;

pub use crate::app::POOL;

pub struct Sponsor{
    pub account_id: String,
    pub sponsor_name: String,
    pub email: String,
    pub phone_number: String,
}

#[inline]
fn format_string(src: &String) -> String {
    if src.len() <= 1{
        return src.clone()
    }
    if src == "NULL"{
        return "".to_string()
    }
    src[1..src.len() - 1].to_string()
}

#[inline]
fn md5_with_salt(id: &String, raw_password: &String) -> String {
    format!("{:x}", md5::compute(raw_password.to_owned() + id))
}

pub fn sponsor_register(id: &String, name: &String, raw_password: &String)
                              -> Result<(), String> {
    let command = format!("INSERT INTO sponsor_account (account_id, sponsor_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id = id, name = name, password = md5_with_salt(id, raw_password));
    println!("{}", command);

    match POOL.prep_exec(command, ()) {
        Ok(_) => Ok(()),
        Err(e) =>  Err(e.to_string()),
    }
}

pub fn sponsor_log_in(id: &String, raw_password: &String)
                            -> Result<String, String> {
    let password = md5_with_salt(&id, &raw_password);
    let command = format!("SELECT password, sponsor_name FROM sponsor_account WHERE account_id='{id}';", id = id);
    //println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {}
    }

    for row in res.unwrap(){
        let result = row.unwrap().unwrap();
        let pwd = format_string(&result[0].as_sql(true));
        println!("{}, {}", password,pwd);
        if password == pwd{
            let name = format_string(&result[1].as_sql(true));
            return Ok(name);
        } else {
            return Err(String::from("Wrong password."));
        }
    }

    return Err(String::from("Account does not exist."));
}

pub fn get_sponsor_events(name: &String, event_list: &mut Vec<Event>)
    -> Result<(), String> {
    let command = format!("SELECT * FROM event WHERE sponsor_name='{name}'", name = name);
    //println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }

    for row in res.unwrap() {
        let ev = row.unwrap().unwrap();
        let event = Event {
            event_id: format_string(&ev[0].as_sql(true)),
            sponsor_name: format_string(&ev[1].as_sql(true)),
            event_name: format_string(&ev[2].as_sql(true)),
            start_time: format_string(&ev[3].as_sql(true)),
            end_time: format_string(&ev[4].as_sql(true)),
            event_type: ev[5].as_sql(true).parse().unwrap(),
            event_introduction: format_string(&ev[6].as_sql(true)),
            event_capacity: ev[7].as_sql(true).parse().unwrap(),
            current_participants: ev[8].as_sql(true).parse().unwrap(),
            left_tickets: ev[9].as_sql(true).parse().unwrap(),
            event_status: ev[10].as_sql(true).parse().unwrap(),
            event_location: format_string(&ev[11].as_sql(true)),
            update_type: 0
        };
        event_list.push(event);
    }
    return Ok(());
}

pub fn check_sponsor_by_id(id: &String)->Result<bool, String>{
    let command = format!("SELECT count(*) FROM sponsor_account WHERE account_id='{id}'", id=id);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => {println!("{}", e.to_string()); return Err(e.to_string())},
        _ => {},
    }
    for row in res.unwrap(){
        let result = row.unwrap().unwrap();
        let num = result[0].as_sql(true);
        if num == "0"{
            return Ok(false);
        }
        else if  num == "1"{
            return Ok(true);
        }
        else{
            return Err("Something wrong in database".to_string());
        }
    }
    return Ok(true);
}

pub fn get_info_by_name(name: &String)->Result<Sponsor, String>{
    let command = format!("SELECT account_id, sponsor_name, email, phone_number From sponsor_account \
     WHERE sponsor_name='{name}';", name=name);
    println!("{}", command);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => {println!("{}", e.to_string()); return Err(e.to_string())},
        _ => {},
    }
    for row in res.unwrap(){
        let info = row.unwrap().unwrap();
        let sponsor = Sponsor{
            account_id: format_string(&info[0].as_sql(true)),
            sponsor_name: format_string(&info[1].as_sql(true)),
            email: format_string(&info[2].as_sql(true)),
            phone_number: format_string(&info[3].as_sql(true))
        };
        return Ok(sponsor);
    }
    return Err("No such sponsor".to_string());
}

pub fn alter_sponsor_info(sponsor: &Sponsor)->Result<(), String>{
    let command = format!("UPDATE sponsor_account SET email='{email}', phone_number='{phone_number}' \
    WHERE sponsor_name='{sponsor_name}';", email=sponsor.email, phone_number=sponsor.phone_number,
                          sponsor_name=sponsor.sponsor_name);
    println!("{}", command);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => {println!("{}", e.to_string()); return Err(e.to_string())},
        Ok(o) =>Ok(()),
    }
}
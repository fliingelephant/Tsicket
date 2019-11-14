extern crate md5;
extern crate mysql;

use std::*;
use std::result::Result;

use serde::{Serialize};

use mysql as my;

use super::events::Event;

pub use crate::app::POOL;

#[inline]
fn format_string(src: &String) -> String {
    src[1..src.len() - 1].to_string()
}

#[inline]
fn md5_with_salt(id: &String, raw_password: &String) -> String {
    format!("{:x}", md5::compute(raw_password.to_owned() + id))
}

pub fn sponsor_register(id: &String, name: &String, raw_password: &String)
                              -> Result<(), String> {
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("INSERT INTO sponsor_account (account_id, sponsor_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id = id, name = name, password = md5_with_salt(id, raw_password));
    //println!("{}", command);

    match pool.prep_exec(command, ()) {
        Ok(_) => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}

pub fn sponsor_log_in(id: &String, raw_password: &String)
                            -> Result<(), String> {
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();

    let password = md5_with_salt(&raw_password,  &id);
    let command = format!("SELECT password FROM sponsor_account WHERE account_id='{id}';", id = id);
    //println!("{}", command);

    let res = pool.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {}
    }

    for row in res.unwrap(){
        let pwd = format_string(&row.unwrap().unwrap()[0].as_sql(true));
        if password == pwd{
            return Ok(());
        } else {
            return Err(String::from("Wrong password."));
        }
    }

    return Err(String::from("Account does not exist."));
}

pub fn get_sponsor_events(name: &String, event_list: &mut Vec<Event>)
    -> Result<(), String> {
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let command = format!("SELECT * FROM event WHERE sponsor_name='{name}'", name = name);
    //println!("{}", command);

    let res = pool.prep_exec(command, ());
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
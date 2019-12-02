extern crate md5;
extern crate mysql;

use std::*;
use std::result::Result;

use serde::{Deserialize, Serialize};

use super::events::Event;
use super::users;

pub use crate::app::POOL;

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

pub fn sponsor_register(
    id: &String, name: &String, raw_password: &String,
    email: &String, phone_number: &String
)-> Result<(), String> {
    let command = format!("INSERT INTO sponsor_account (account_id, sponsor_name, password,\
     email, phone_number) VALUES ('{id}', '{name}', '{password}', '{email}', '{phone_number}');",
                          id = id, name = name, password = md5_with_salt(id, raw_password),
                          email=email, phone_number=phone_number);
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

pub fn get_sponsor_events(name: &String)-> Result<Vec<Event>, String> {
    let command = format!("SELECT * FROM event WHERE sponsor_name='{name}'", name = name);
    //println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }

    let mut event_list:Vec<Event> = Vec::new();
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
            event_picture: format_string(&ev[7].as_sql(true)),
            event_capacity: ev[8].as_sql(true).parse().unwrap(),
            current_participants: ev[9].as_sql(true).parse().unwrap(),
            left_tickets: ev[10].as_sql(true).parse().unwrap(),
            event_status: ev[11].as_sql(true).parse().unwrap(),
            event_location: format_string(&ev[12].as_sql(true)),
            update_type: 0
        };
        event_list.push(event);
    }
    return Ok(event_list);
}

pub fn check_sponsor_by_id(id: &String)->Result<bool, String>{
    let command = format!("SELECT count(*) FROM sponsor_account WHERE account_id='{id}'", id=id);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
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

#[derive(Deserialize, Serialize)]
pub struct Sponsor{
    pub id: String,
    pub sponsor_name: String,
    pub head_portrait: String,
    pub email: String,
    pub phone_number: String,
}

pub fn get_info_by_name(name: &String)->Result<Sponsor, String>{
    let command = format!("SELECT account_id, sponsor_name, head_portrait, email, phone_number From sponsor_account \
     WHERE sponsor_name='{name}';", name=name);
    println!("{}", command);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    for row in res.unwrap(){
        let info = row.unwrap().unwrap();
        let sponsor = Sponsor{
            id: format_string(&info[0].as_sql(true)),
            sponsor_name: format_string(&info[1].as_sql(true)),
            head_portrait: format_string(&info[2].as_sql(true)),
            email: format_string(&info[3].as_sql(true)),
            phone_number: format_string(&info[4].as_sql(true))
        };
        return Ok(sponsor);
    }
    return Err("No such sponsor".to_string());
}

pub fn alter_sponsor_info(
    sponsor: &Sponsor
) -> Result<(), String> {
    let command = format!("UPDATE sponsor_account SET head_portrait='{head}', email='{email}', phone_number='{phone_number}' \
    WHERE sponsor_name='{sponsor_name}';", email=sponsor.email, phone_number=sponsor.phone_number,
                          head=sponsor.head_portrait, sponsor_name=sponsor.sponsor_name);
    println!("{}", command);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        Ok(o) =>Ok(()),
    }
}

pub fn publish_moment(sponsor_name: &String, event_id: &String, text: &String, pictures: &Vec<String>)
                       -> Result<(), String>{
    let mut picture_str = "".to_string();
    for pic in pictures{
        if picture_str == ""{
            picture_str = picture_str + &pic;
        }
        else{
            picture_str = picture_str + "&" + &pic;
        }
    }
    let command = format!("INSERT INTO moment (sponsor_name, event_id, text, pictures) VALUES \
     ('{sponsor_name}', '{event_id}', '{text}', '{pictures}');", sponsor_name=sponsor_name,
                          event_id=event_id, text=text, pictures=picture_str);
    println!("{}", command);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        Ok(o) =>Ok(()),
    }
}

pub fn release_push(sponsor_name: &String, event_id: &String, text: &String)
                      -> Result<(), String>{
    let res = users::get_users_by_event_id(event_id);
    match res{
        Err(e) => return Err(e),
        _  => {},
    }
    let users = res.ok().unwrap();
    for user in users{
        let command = format!("INSERT INTO push (sponsor_name, event_id, user_id, text) VALUES \
     ('{sponsor_name}', '{event_id}', '{user_id}', '{text}');", sponsor_name=sponsor_name,
                              event_id=event_id, user_id=user, text=text);
        println!("{}", command);
        let res = POOL.prep_exec(command, ());
        match res {
            Err(e) => return Err(e.to_string()),
            Ok(o) => {},
        }
    }
    return Ok(());
}
/*extern crate mysql;

use mysql as my;
use std::*;
use std::string::ToString;

fn format_string(mut src: String)->String{
    let len = src.len();
    let rlt = src[1..len-1].to_string();
    return rlt;
}*/

/*
pub fn user_sign_up(id: &str, name: &str, password: &str)->bool{
    TODO: for test
    let pool = my::Pool::new("mysql://root:T%i8c3k8E%23t5@localhost:3306/tsicket").unwrap();
    let mut command = "INSERT INTO user_account (account_id, username, password) VALUES (".to_string();
    command = command + "'" + id + "',";
    command = command + "'" + name + "',";
    command = command + "'" + password + "');";
    //println!("{}", command);
    let req = pool.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return true,
        Result::Err(_err)=> return false,
    }

} */

pub use crate::app::POOL;
use crate::db::records::Record;

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
pub fn add_user(id: &String)->Result<(), String>{
    let command = format!("INSERT INTO user_account (account_id) VALUES ('{id}');", id=id);
    println!("{}", command);

    match POOL.prep_exec(command, ()) {
        Ok(_) => Ok(()),
        Err(e) =>  Err(e.to_string()),
    }
}

#[inline]
pub fn set_tsinghua_id(id: &String, tsinghua_id: &String)->Result<(), String>{
    let command = format!("UPDATE user_account SET tsinghua_id='{tsinghua_id}' \
    WHERE account_id='{id}';", tsinghua_id=tsinghua_id, id=id);
    println!("{}", command);

    match POOL.prep_exec(command, ()) {
        Ok(_) => Ok(()),
        Err(e) =>  Err(e.to_string()),
    }
}

pub fn check_user_by_id(id: &String)->Result<bool, String>{
    let command = format!("select count(*) from user_account where account_id='{id}';", id=id);

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

pub fn check_tsinghua_id(id: &String)->Result<bool, String>{
    let command = format!("SELECT tsinghua_id FROM user_account WHERE account_id='{id}';", id=id);
    println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    for row in res.unwrap(){
        let result = row.unwrap().unwrap();
        let tsinghua_id = format_string(&result[0].as_sql(true));
        if tsinghua_id.is_empty(){
            return Ok(false);
        }
        else {
            return Ok(true);
        }
    }
    return Err("No such user".to_string());
}

pub fn get_tsinghua_id(id: &String) -> Result<String, String> {
    let command = format!("SELECT tsinghua_id FROM user_account WHERE account_id='{id}';", id=id);
    println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    for row in res.unwrap(){
        let result = row.unwrap().unwrap();
        let tsinghua_id = format_string(&result[0].as_sql(true));
        if tsinghua_id.is_empty(){
            return Err("Tsinghua ID is not set yet.".to_string());
        }
        else {
            return Ok(tsinghua_id.clone());
        }
    }
    return Err("No such user".to_string());
}

#[allow(dead_code)]
pub fn get_user_records(id: &String)
                          -> Result<Vec<Record>, String> {
    let command = format!("SELECT * FROM ticket_record WHERE user_id='{id}';", id = id);
    //println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }

    let mut record_list:Vec<Record> = Vec::new();
    for row in res.unwrap() {
        let re = row.unwrap().unwrap();
        let record = Record {
            record_id: format_string(&re[0].as_sql(true)),
            event_id: format_string(&re[1].as_sql(true)),
            sponsor_name: format_string(&re[2].as_sql(true)),
            user_id: format_string(&re[3].as_sql(true)),
            start_time: format_string(&re[4].as_sql(true)),
            end_time: format_string(&re[5].as_sql(true)),
            update_type: -1,
        };
        record_list.push(record);
    }
    return Ok(record_list);
}

pub fn get_user_likes(id: &String) -> Result<Vec<String>, String>{
    let command = format!("SELECT event_id FROM `like` WHERE user_id='{id}';", id = id);
    println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }

    let mut event_list:Vec<String> = Vec::new();
    for row in res.unwrap() {
        let re = row.unwrap().unwrap();
        let event = format_string(&re[0].as_sql(true));
        event_list.push(event);
    }
    return Ok(event_list);
}

pub fn get_user_like_number(id: &String) -> Result<i32, String>{
    let command = format!("SELECT COUNT(*) FROM `like` WHERE user_id='{id}';", id = id);
    println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    for row in res.unwrap() {
        let info = row.unwrap().unwrap();
        let number: i32 = info[0].as_sql(true).parse().unwrap();
        return Ok(number);
    }
    return Ok(0);
}

pub fn check_user_like(user_id: &String, event_id: &String) -> Result<bool, String> {
    let res = get_user_likes(user_id);
    match res{
        Err(e) => return Err(e.to_string()),
        Ok(likes) => {
            return Ok(likes.contains(event_id))
        },
    }
}

pub fn set_user_like(user_id: &String, event_id: &String) -> Result<(), String> {
    let command = format!("INSERT INTO `like` (user_id, event_id) VALUES \
    ('{user_id}', '{event_id}');", user_id=user_id, event_id=event_id);
    println!("{}", command);

    match POOL.prep_exec(command, ()){
        Err(e) => return Err(e.to_string()),
        Ok(_) => return Ok(()),
    }
}

pub fn cancel_user_like(user_id: &String, event_id: &String) -> Result<(), String> {
    let command = format!("DELETE FROM `like` WHERE (user_id, event_id)=\
    ('{user_id}', '{event_id}');", user_id=user_id, event_id=event_id);
    println!("{}", command);

    match POOL.prep_exec(command, ()){
        Err(e) => return Err(e.to_string()),
        Ok(_) => return Ok(()),
    }
}

pub fn get_user_follows(id: &String) -> Result<Vec<String>, String>{
    let command = format!("SELECT sponsor_name FROM `follow` WHERE user_id='{id}';", id = id);
    println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }

    let mut sponsor_list:Vec<String> = Vec::new();
    for row in res.unwrap() {
        let re = row.unwrap().unwrap();
        let sponsor = format_string(&re[0].as_sql(true));
        sponsor_list.push(sponsor);
    }
    return Ok(sponsor_list);
}

pub fn get_user_follow_number(id: &String) -> Result<i32, String>{
    let command = format!("SELECT COUNT(*) FROM follow WHERE user_id='{id}';", id = id);
    println!("{}", command);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    for row in res.unwrap() {
        let info = row.unwrap().unwrap();
        let number: i32 = info[0].as_sql(true).parse().unwrap();
        return Ok(number);
    }
    return Ok(0);
}

pub fn check_user_follow(user_id: &String, sponsor_name: &String) -> Result<bool, String> {
    let res = get_user_follows(user_id);
    match res{
        Err(e) => return Err(e.to_string()),
        Ok(likes) => {
            return Ok(likes.contains(sponsor_name))
        },
    }
}

pub fn set_user_follow(user_id: &String, sponsor_name: &String) -> Result<(), String> {
    let command = format!("INSERT INTO `follow` (user_id, sponsor_name) VALUES \
    ('{user_id}', '{sponsor_name}');", user_id=user_id, sponsor_name=sponsor_name);
    println!("{}", command);

    match POOL.prep_exec(command, ()){
        Err(e) => return Err(e.to_string()),
        Ok(_) => return Ok(()),
    }
}

pub fn cancel_user_follow(user_id: &String, sponsor_name: &String) -> Result<(), String> {
    let command = format!("DELETE FROM `follow` WHERE (user_id, sponsor_name)=\
    ('{user_id}', '{sponsor_name}');", user_id=user_id, sponsor_name=sponsor_name);
    println!("{}", command);

    match POOL.prep_exec(command, ()){
        Err(e) => return Err(e.to_string()),
        Ok(_) => return Ok(()),
    }
}

pub fn get_users_by_event_id(event_id: &String) -> Result<Vec<String>, String>{
    let command = format!("SELECT user_id From ticket_record WHERE event_id='{id}';", id=event_id);
    println!("{}", command);
    let mut users:Vec<String> = Vec::new();
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        Ok(o) => {
            for row in o {
                let elements = row.unwrap().unwrap();
                let user = format_string(&elements[0].as_sql(true));
                users.push(user);
            }
            return Ok(users);
        }
    }
}
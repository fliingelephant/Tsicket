use serde::{Serialize};

pub use crate::app::POOL;
use crate::db::users;

#[derive(Serialize)]
pub struct Notification {
    pub sponsor_name: String,
    pub event_id: String,
    pub user_id: String,
    pub notice_id: String,
    pub text: String,
    pub time: String,
    pub read: i8,
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

pub fn release_notification(sponsor_name: &String, event_id: &String, notice_id: &String, text: &String)
                    -> Result<(), String>{
    let res = users::get_users_by_event_id(event_id);
    match res{
        Err(e) => return Err(e),
        _  => {},
    }
    let users = res.ok().unwrap();
    for user in users{
        let command = format!("INSERT INTO push (sponsor_name, event_id, user_id, notice_id, text\
        ) VALUES ('{sponsor_name}', '{event_id}', '{user_id}', '{notice_id}', '{text}');",
                              sponsor_name=sponsor_name, event_id=event_id,
                              user_id=user, notice_id=notice_id, text=text);
        let res = POOL.prep_exec(command, ());
        match res {
            Err(e) => return Err(e.to_string()),
            Ok(_) => {},
        }
    }
    return Ok(());
}

pub fn release_notification_specific(sponsor_name: &String, event_id: &String, user_id: &String,
                                     notice_id: &String, text: &String) -> Result<(), String> {
    let command = format!("INSERT INTO push (sponsor_name, event_id, user_id, notice_id, text\
        ) VALUES ('{sponsor_name}', '{event_id}', '{user_id}', '{notice_id}', '{text}');",
                          sponsor_name=sponsor_name, event_id=event_id,
                          user_id=user_id, notice_id=notice_id, text=text);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {},
    }
    return Ok(());
}

pub fn get_user_notifications(user_id: &String)->Result<Vec<Notification>, String>{
    let command = format!("SELECT * FROM notification WHERE user_id='{user_id}';", user_id=user_id);
    println!("{}", command);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    let mut notices:Vec<Notification> = Vec::new();
    for row in res.unwrap(){
        let info = row.unwrap().unwrap();
        let notice = Notification{
            sponsor_name: format_string(&info[0].as_sql(true)),
            event_id: format_string(&info[1].as_sql(true)),
            user_id: format_string(&info[2].as_sql(true)),
            notice_id: format_string(&info[3].as_sql(true)),
            text: format_string(&info[4].as_sql(true)),
            time: format_string(&info[5].as_sql(true)),
            read: info[6].as_sql(true).parse().unwrap()
        };
        notices.push(notice);
    }
    return Ok(notices);
}
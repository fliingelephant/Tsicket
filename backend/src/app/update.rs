use crate::db::events;
use crate::db::records;
pub use crate::app::POOL;
use super::{EVENT_LIST, RECORD};
use chrono::prelude::*;

pub fn update(){
    update_events();
    //update_records();
}

pub fn update_events()
    -> Result<(), String> {
    match (*EVENT_LIST).try_lock() {
        Ok(a) => {drop(a);}
        Err(e) => {println!("{:?}", e);}
    }
    let mut events = (*EVENT_LIST).lock().unwrap();
    for mut event in events.values_mut() {
        
        let start_time;
        if event.start_time.len() == 10 {
            if event.end_time.chars().nth(4) == Some('-') {
                start_time = Local.datetime_from_str(&(event.start_time.clone() + " 00:00:00+0800"), "%Y-%m-%d %H:%M:%S%z").unwrap();
            } else {
                start_time = Local.datetime_from_str(&(event.start_time.clone() + " 00:00:00+0800"), "%Y/%m/%d %H:%M:%S%z").unwrap();
            }
        } else {
            if event.end_time.chars().nth(4) == Some('-') {
                start_time = Local.datetime_from_str(&(event.start_time.clone() + "+0800"), "%Y-%m-%d %H:%M:%S%z").unwrap();
            } else {
                start_time = Local.datetime_from_str(&(event.start_time.clone() + "+0800"), "%Y/%m/%d %H:%M:%S%z").unwrap();
            }
        }
        let end_time;
        if event.end_time.len() == 10 {
            if event.end_time.chars().nth(4) == Some('-') {
                end_time = Local.datetime_from_str(&(event.end_time.clone() + " 00:00:00+0800"), "%Y-%m-%d %H:%M:%S%z").unwrap();
            } else {
                end_time = Local.datetime_from_str(&(event.end_time.clone() + " 00:00:00+0800"), "%Y/%m/%d %H:%M:%S%z").unwrap();
            }
        } else {
            if event.end_time.chars().nth(4) == Some('-') {
                end_time = Local.datetime_from_str(&(event.end_time.clone() + "+0800"), "%Y-%m-%d %H:%M:%S%z").unwrap();
            } else {
                end_time = Local.datetime_from_str(&(event.end_time.clone() + "+0800"), "%Y/%m/%d %H:%M:%S%z").unwrap();
            }
        }
        let now = Local::now();
        if (end_time < now) && (event.event_status % 10 <= 2) && (event.event_status % 10 >= 1) {
            let status = event.event_status % 10;
            event.event_status += 3 - status;
            if event.update_type != 2 {
                event.update_type = 1;
            }
        } else if (start_time<= now) && (end_time > now) && (event.event_status % 10 == 1) {
            event.event_status += 1;
            if event.update_type != 2 {
                event.update_type = 1;
            }
        }

        if event.update_type == 1 { // 修改
            let command = format!("UPDATE event SET sponsor_name='{sponsor_name}', event_name='{event_name}', \
                                    start_time='{start_time}', end_time='{end_time}', event_type={event_type}, \
                                    event_introduction='{event_introduction}', event_picture='{event_picture}', event_capacity={event_capacity}, \
                                    current_participants={current_participants}, left_tickets={left_tickets}, \
                                    event_status={event_status}, event_location='{event_location}', event_time='{event_time}' \
                                    WHERE event_id='{event_id}';", sponsor_name=event.sponsor_name,
                                  event_name=event.event_name, start_time=event.start_time,
                                  end_time=event.end_time, event_type=event.event_type,
                                  event_introduction=event.event_introduction, event_picture=event.event_picture, event_capacity=event.event_capacity,
                                  current_participants=event.current_participants, left_tickets=event.left_tickets,
                                  event_status=event.event_status, event_location=event.event_location, event_time=event.event_time,
                                  event_id=event.event_id);
            println!("{}", command);
            let req=POOL.prep_exec(command, ());
            match req {
                Result::Err(e) => {
                    println!("{}", e.to_string());
                    return Err(e.to_string());
                }
                _ => {}
            }
            event.update_type = 0;
        } else if event.update_type == 2 { // 添加
            let command = format!("INSERT INTO event (event_id, sponsor_name, event_name, start_time, end_time, \
                                    event_type, event_introduction, event_picture, event_capacity, current_participants, \
                                    left_tickets, event_status, event_location, event_time) VAlUES ('{event_id}', \
                                    '{sponsor_name}', '{event_name}', '{start_time}', '{end_time}', {event_type}, \
                                    '{event_introduction}', '{event_picture}', {event_capacity}, {current_participants}, \
                                    {left_tickets}, {event_status}, '{event_location}', '{event_time}');", event_id=event.event_id,
                                  sponsor_name=event.sponsor_name, event_name=event.event_name,
                                  start_time=event.start_time,
                                  end_time=event.end_time, event_type=event.event_type,
                                  event_introduction=event.event_introduction,
                                  event_picture=event.event_picture, event_capacity=event.event_capacity,
                                  current_participants=event.current_participants, left_tickets=event.left_tickets,
                                  event_status=event.event_status, event_location=event.event_location,
                                  event_time=event.event_time);
            let req = POOL.prep_exec(command, ());
            match req {
                Result::Err(e) => {
                    println!("{}", e.to_string());
                    return Err(e.to_string());
                }
                _ => {}
            }
            event.update_type = 0;
        }
    }
    Ok(())
}

pub fn update_records()
    -> Result<(), String> {
    match (*RECORD).try_lock() {
        Ok(a) => {println!("OKOKOK!");drop(a);}
        Err(e) => {println!("{:?}", e);}
    }
    let mut records = (*RECORD).lock().unwrap();
    let mut to_remove: Vec<String> = vec![];
    for (index_str, record) in records.iter_mut() {
        /*
        if record.update_type == 1 { //修改
            let command = format!("UPDATE ticket_record SET event_id='{event_id}', \
                                    sponsor_name='{sponsor_name}', user_id='{user_id}',\
                                     start_time='{start_time}', end_time='{end_time}'\
                                     WHERE record_id='{record_id}';",
                                  event_id=record.event_id, sponsor_name=record.sponsor_name,
                                  user_id=record.user_id, start_time=record.start_time,
                                  end_time=record.end_time,record_id=record.record_id);
            let req=POOL.prep_exec(command, ());
            match req {
                Result::Err(_err) => println!("{}", _err.to_string()),
                _ => {}
            }
            record.update_type = 0;
        }
        else */
        if record.update_type == 1 { //增加
            let command = format!("INSERT INTO ticket_record (record_id, event_id, sponsor_name, user_id,\
                                    start_time, end_time) VAlUES ('{record_id}', '{event_id}', \
                                    '{sponsor_name}', '{user_id}', '{start_time}', '{end_time}');",
                                  record_id=record.record_id, event_id=record.event_id,
                                  sponsor_name=record.sponsor_name, user_id=record.user_id,
                                  start_time=record.start_time,
                                  end_time=record.end_time);
            println!("{}", command);
            let req = POOL.prep_exec(command, ());
            match req {
                Err(e) => {
                    println!("{}", e.to_string());
                    return Err(e.to_string());
                }
                _ => {}
            }
            record.update_type = 0;
        }
        else if record.update_type == 2 { //删除
            let command = format!("DELETE FROM ticket_record WHERE record_id='{record_id}';", record_id=record.record_id);
            println!("{}", command);
            let req = POOL.prep_exec(command, ());
            match req {
                Result::Err(e) => {
                    println!("{}", e.to_string());
                    return Err(e.to_string());
                }
                _ => {}
            }
            to_remove.push(record.record_id.clone());
        }
    }
    {
        for id in to_remove {
            records.remove(&id.clone());
        }
    }
    Ok(())
}
use serde::{Deserialize, Serialize};

use mysql as my;

pub use crate::app::POOL;

#[inline]
fn format_string(src: &String) -> String {
    src[1..src.len() - 1].to_string()
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Event {
    pub event_id: String,
    pub sponsor_name: String,
    pub event_name: String,
    pub start_time: String,
    pub end_time: String,
    pub event_type: i8,
    pub event_introduction: String,
    pub event_capacity: i32,
    pub current_participants: i32,
    pub left_tickets: i32,
    pub event_status: i8,
    pub event_location: String,
    pub update_type: i8,
}

pub fn get_sponsor_events(name: &String, event_list: &mut Vec<Event>)
                          ->Result<(), String> {
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
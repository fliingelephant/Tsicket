
use crate::app::POOL;

use crate::db::events::Event;
use crate::db::records::Record;

#[inline]
fn format_string(src: &String) -> String {
    src[1..src.len() - 1].to_string()
}

pub fn initiate(event_list: &mut Vec<Event>)-> Result<(), String> {
    let command_event = "SELECT * FROM event;".to_string();
    let res = POOL.prep_exec(command_event, ());
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

    /*let command_record = "SELECT * FROM ticket_record;".to_string();
    let res = POOL.prep_exec(command_record, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    for row in res.unwrap() {
        let ev = row.unwrap().unwrap();
        let record = Record {
            record_id: format_string(&ev[0].as_sql(true)),
            event_id: format_string(&ev[1].as_sql(true)),
            sponsor_name: format_string(&ev[2].as_sql(true)),
            user_id: format_string(&ev[3].as_sql(true)),
            start_time: format_string(&ev[4].as_sql(true)),
            end_time: format_string(&ev[5].as_sql(true)),
            update_type: 0
        };
        record_list.push(record);
    }*/

    return Ok(());
}
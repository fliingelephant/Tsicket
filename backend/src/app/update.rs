use crate::db::events;
use crate::db::records;
pub use crate::app::POOL;

pub fn update(events: &mut Vec<events::Event>, records: &mut Vec<records::Record>){
    update_events(events.as_mut());
    update_records(records.as_mut());
}

fn update_events(events: &mut Vec<events::Event>){
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
                                  start_time=event.start_time,
                                  end_time=event.end_time, event_type=event.event_type,
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

fn update_records(records: &mut Vec<records::Record>){
    for record in records{
        /*if record.update_type == 1 { //修改
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
            let req = POOL.prep_exec(command, ());
            match req {
                Result::Err(_err) => println!("{}", _err.to_string()),
                _ => {}
            }
            record.update_type = 0;
        }
        else if record.update_type == 2 { //删除
            let command = format!("DELETE FROM ticket_record WHERE record_id='{record_id}';", record_id=record.record_id);
            let req = POOL.prep_exec(command, ());
            match req {
                Result::Err(_err) => println!("{}", _err.to_string()),
                _ => {}
            }
            record.update_type = 0;
            //如何删除vec中的指定项？
        }
    }
}
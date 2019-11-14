use time;

use crate::db::events;
pub use crate::app::POOL;

#[inline]
fn str_to_timestamp(str: String)->i64{
    let tm = time::strptime(&str, "%Y-%m-%d %H:%M:%S");
    return tm.unwrap().to_timespec().sec;
}

fn update(events: Vec<events::Event>){
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
            println!("{}", command);
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
                                  start_time=str_to_timestamp(event.start_time),
                                  end_time=str_to_timestamp(event.end_time), event_type=event.event_type,
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
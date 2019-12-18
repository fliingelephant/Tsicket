use crate::app::admins::{EventInfoRetToAdmin};
use crate::app::POOL;

#[inline]
fn format_string(src: &String) -> String {
    src[1..src.len() - 1].to_string()
}

fn admin_sign_up(id: String, name: String, raw_password: String)->String{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("INSERT INTO admin_account (account_id, admin_name, password) VALUES\
     ('{id}', '{name}', '{password}');", id=id, name=name, password=password);
    let req = POOL.prep_exec(command, ());
    match req {
        Result::Ok(_val) => return "success".to_string(),
        Result::Err(_err)=> return _err.to_string(),
    }
}

fn admin_log_in(id :String, raw_password: String)->Result<(), String>{
    let password = format!("{:x}", md5::compute(raw_password + &id));
    let command = format!("SELECT password FROM admin_account WHERE account_id='{id}';", id=id);
    //println!("{}", command);
    let req = POOL.prep_exec(command, ());
    match req {
        Err(e) => return Err(e.to_string()),
        _ => {}
    }
    for row in req.unwrap(){
        let pwd = format_string(&row.unwrap().unwrap()[0].as_sql(true));
        if password == pwd{
            return Ok(());
        } else {
            return Err("Wrong password".to_string());
        }
    }
    return Err("No such admin".to_string());
}

pub fn get_all_events(
    event_list: &mut Vec<EventInfoRetToAdmin>
) -> Result<(), String> {
    let command_event = "SELECT * FROM event;".to_string();
    let res = POOL.prep_exec(command_event, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }

    for row in res.unwrap() {
        let ev = row.unwrap().unwrap();
        let event = EventInfoRetToAdmin {
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
            event_time: format_string(&ev[13].as_sql(true))
        };
        event_list.push(event);
    }
    Ok(())
}

pub fn cancel_event(
    event_id: &String
) -> Result<(), String> {
    let command = format!("UPDATE event SET event_status=4 WHERE event_id='{event_id}';",
        event_id=event_id);
    println!("{}", command);
    let req=POOL.prep_exec(command, ());
    match req {
        Err(e) => Err(e.to_string()),
        Ok(_) => Ok(())
    }
}
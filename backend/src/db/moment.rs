use serde::{Serialize};

pub use crate::app::POOL;

#[derive(Serialize)]
pub struct Moment {
     pub sponsor_name: String,
     pub event_id: String,
     pub moment_id: String,
     pub text: String,
     pub pictures: Vec<String>,
     pub time: String,
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

pub fn publish_moment(
    sponsor_name: &String,
    event_id: &String,
    moment_id: &String,
    text: &String,
    pictures: &Vec<String>
) -> Result<(), String> {
    let mut picture_str = "".to_string();
    for pic in pictures {
        if picture_str == "" {
            picture_str = picture_str + &pic;
        } else {
            picture_str = picture_str + "&" + &pic;
        }
    }
    let command = format!("INSERT INTO moment (sponsor_name, event_id, moment_id, text, pictures\
    ) VALUES ('{sponsor_name}', '{event_id}', '{moment_id}', '{text}', '{pictures}');", sponsor_name
    = sponsor_name, event_id = event_id, moment_id = moment_id, text = text, pictures = picture_str);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        Ok(o) => Ok(()),
    }
}

pub fn get_event_moments(event_id: &String)->Result<Vec<Moment>, String>{
     let command = format!("SELECT * FROM moment WHERE event_id='{event_id}';", event_id=event_id);
     println!("{}", command);
     let res = POOL.prep_exec(command, ());
     match res {
         Err(e) => return Err(e.to_string()),
         _ => {},
     }
     let mut moments:Vec<Moment> = Vec::new();
     for row in res.unwrap(){
         let info = row.unwrap().unwrap();
         let pcs = format_string(&info[4].as_sql(true));
         let pts: Vec<&str> = pcs.split('&').collect();
         let mut pictures: Vec<String> = Vec::new();
         for pc in pts{
             pictures.push(pc.to_string());
         }
         let moment = Moment{
             sponsor_name: format_string(&info[0].as_sql(true)),
             event_id: format_string(&info[1].as_sql(true)),
             moment_id: format_string(&info[2].as_sql(true)),
             text: format_string(&info[3].as_sql(true)),
             pictures,
             time: format_string(&info[5].as_sql(true))
         };
         moments.push(moment);
     }
     return Ok(moments);
}

pub fn get_sponsor_moments(sponsor_name: &String)->Result<Vec<Moment>, String>{
    let command = format!("SELECT * FROM moment WHERE sponsor_name='{sponsor_name}';", sponsor_name=sponsor_name);
    println!("{}", command);
    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => return Err(e.to_string()),
        _ => {},
    }
    let mut moments:Vec<Moment> = Vec::new();
    for row in res.unwrap(){
        let info = row.unwrap().unwrap();
        let pcs = format_string(&info[3].as_sql(true));
        let pts: Vec<&str> = pcs.split('&').collect();
        let mut pictures: Vec<String> = Vec::new();
        for pc in pts{
            pictures.push(pc.to_string());
        }
        let moment = Moment{
            sponsor_name: format_string(&info[0].as_sql(true)),
            event_id: format_string(&info[1].as_sql(true)),
            moment_id: format_string(&info[2].as_sql(true)),
            text: format_string(&info[3].as_sql(true)),
            pictures,
            time: format_string(&info[5].as_sql(true))
        };
        moments.push(moment);
    }
    return Ok(moments);
}
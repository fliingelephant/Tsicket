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


pub fn check_user_by_id(id: &String)->Result<bool, String>{
    let command = format!("select count(*) from user_account where account_id='{id}'", id=id);

    let res = POOL.prep_exec(command, ());
    match res {
        Err(e) => {println!("{}", e.to_string()); return Err(e.to_string())},
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
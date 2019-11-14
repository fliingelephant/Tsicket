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
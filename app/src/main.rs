use mysql::serde_json::value::Value::Null;
include!("db_api.rs");



fn main() {
    println!("result of user sign up: {}", user_sign_up("999", "zjr"));

    println!("result of sponsor sign up: {}", sponsor_sign_up("123", "1", "123456"));
    //let events = get_sponsor_events("123");
    //println!("{}", events[0].event_introduction); 还没写添加活动

    println!("result of sponsor successful log in : {}", sponsor_log_in("123", "123456"));
    println!("result sponsor log in with wrong password: {}", sponsor_log_in("123", "000000"));
    println!("result sponsor log in with invalid id: {}", sponsor_log_in("0", "123456"));

    println!("result of admin sign up: {}", admin_sign_up("0011", "yzj", "123"));
    println!("result of admin successful log in : {}", admin_log_in("0011", "123"));
    println!("result admin log in with wrong password: {}", admin_log_in("0011", "321"));
    println!("result admin log in with invalid id: {}", admin_log_in("0", "123"));
}
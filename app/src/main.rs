include!("db_api.rs");



fn main() {
    println!("{}", user_sign_up("0011", "yzj", "123"));
    let events = get_publisher_events("123");
    println!("{}", events[0].event_introduction);
}
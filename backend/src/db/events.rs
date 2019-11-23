use serde::{Deserialize, Serialize};

use mysql as my;

pub use crate::app::POOL;

#[inline]
fn format_string(src: &String) -> String {
    src[1..src.len() - 1].to_string()
}

#[derive(Clone, Debug, Deserialize, Serialize)]
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
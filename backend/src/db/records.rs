use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct Record {
    pub record_id: String,
    pub event_id: String,
    pub sponsor_name: String,
    pub user_id: String,
    pub start_time: String,
    pub end_time: String,
    pub update_type: i8,
}

#[derive(Serialize)]
pub struct MomentRecord {
    pub record_id: String,
    pub event_id: String,
    pub sponsor_name: String,
    // TODO: 确定数据格式
    pub content: String,
    pub update_type: i8,
}

#[derive(Serialize)]
pub struct PostRecord {
    pub record_id: String,
    pub event_id: String,
    pub sponsor_name: String,
    // TODO: 确定数据格式
    pub content: String,
    pub update_type: i8,
}
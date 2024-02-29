use crate::resource::Resource;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Block {
    seq: u32,
    length: f32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    resources: Vec<Resource>,
}

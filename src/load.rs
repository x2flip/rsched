use crate::resource::Resource;
use chrono::NaiveDateTime;

#[derive(Debug)]
pub struct Block {
    seq: u32,
    length: i64,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    resources: Vec<Resource>,
}

impl Block {
    pub fn create(
        length: i64,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        resources: Vec<Resource>,
    ) -> Self {
        Block {
            seq: 1,
            length,
            start_time,
            end_time,
            resources,
        }
    }
}

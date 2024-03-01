use std::ops::Add;

use chrono::{Duration, NaiveDate, NaiveTime, Timelike, Utc};

use crate::load::Block;
use crate::resource::Resource;

#[derive(Debug)]
pub struct Operation {
    pub seq: u32,
    code: String,
    resource: Resource,
    length: i64, // In minutes
    blocks: Vec<Block>,
}

#[allow(dead_code, unused_mut, unused_variables)]
impl Operation {
    pub fn schedule_op(&mut self) {
        let minutes = Duration::minutes(self.length.to_owned());
        let mut op_start_date = Utc::today().naive_utc();
        let mut op_start_time = Utc::now().time();
        let mut op_end_date = Utc::today().naive_utc();
        let mut op_end_time = Utc::now().time();
        // Search the required resource for the first available time
        for i in self.resource.capacity.iter() {
            // Check that capacity is after today
            if i.date >= Utc::today().naive_utc() {
                for avail_block in i.availability.iter() {
                    op_start_date = i.date;
                    op_end_date = i.date;
                    op_start_time = avail_block.start;
                    let temp_op_end = op_start_time.add(minutes);
                    if temp_op_end <= avail_block.end {
                        op_end_time = temp_op_end;
                    }
                }
            }
        }

        let res = self.resource.clone();
        let vec_of_resources = vec![res];

        let newBlock = Block::create(
            self.length,
            op_start_date.and_time(op_start_time),
            op_end_date.and_time(op_end_time),
            vec_of_resources,
        );
        self.blocks.push(newBlock);
    }

    pub fn create(seq: u32, code: String, resource: Resource, length: i64) -> Operation {
        Operation {
            seq,
            code,
            resource,
            length,
            blocks: vec![],
        }
    }
}

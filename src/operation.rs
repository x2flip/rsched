use chrono::Duration;

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
    pub fn schedule_op(&self) {
        let mut blocks: Vec<Block> = vec![];
        let minutes = Duration::minutes(self.length.to_owned());
        let operation_length = minutes;
        println!("{:?}", operation_length);
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

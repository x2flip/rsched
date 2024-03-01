use crate::{operation::Operation, resource::Resource};
use chrono::NaiveDate;

#[derive(Debug)]
pub struct Job {
    job_num: String,
    need_date: NaiveDate,
    ops: Vec<Operation>,
}

#[allow(dead_code)]
impl Job {
    pub fn create(job_num: String, need_date: NaiveDate) -> Job {
        Job {
            job_num,
            need_date,
            ops: vec![],
        }
    }

    fn add_operations(&mut self, operations: Vec<Operation>) {
        operations.into_iter().for_each(|op| self.ops.push(op))
    }

    pub fn schedule_job(&mut self) {
        self.ops.iter().for_each(|op| op.schedule_op())
    }

    fn get_latest_operation(&self) -> Option<&Operation> {
        self.ops.first()
    }

    fn get_latest_op_seq(&self) -> u32 {
        match self.get_latest_operation() {
            Some(v) => v.seq,
            None => 0,
        }
    }

    fn get_next_seq(&self) -> u32 {
        self.get_latest_op_seq() + 1
    }

    pub fn add_operation(&mut self, code: String, resource: Resource, minutes: i64) {
        // Get the next sequence number for the operation
        let next_sequence = self.get_next_seq();

        // Create the operation
        let operation = Operation::create(next_sequence, code, resource, minutes);

        // Push the operation into the job
        self.ops.push(operation)
    }
}

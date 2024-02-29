use chrono::{NaiveDate, NaiveTime};

#[derive(Debug)]
pub struct Capacity {
    pub date: NaiveDate,
    pub availability: Vec<AvailabilityBlock>,
}

#[derive(Debug)]
pub struct AvailabilityBlock {
    pub start: NaiveTime,
    pub end: NaiveTime,
}

impl Capacity {
    pub fn get_first_availability(&self) -> Option<&AvailabilityBlock> {
        self.availability.get(0)
    }
}

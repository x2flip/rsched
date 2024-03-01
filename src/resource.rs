use crate::capacity::AvailabilityBlock;
use chrono::{DateTime, Days, Duration, Local, NaiveTime, Utc};

use crate::capacity::Capacity;

#[derive(Debug, Clone)]
pub enum ResourceType {
    Person,
    Machine,
    Pallet,
}

#[derive(Debug, Clone)]
pub struct Resource {
    id: String,
    resource_type: ResourceType,
    pub capacity: Vec<Capacity>,
}

impl Resource {
    pub fn create(id: String, resource_type: ResourceType, capacity_days: Option<u64>) -> Resource {
        let mut res = Resource {
            id,
            resource_type,
            capacity: Vec::new(),
        };

        res.calculate_capacity(capacity_days.unwrap_or(2));
        res
    }

    fn calculate_capacity(&mut self, days: u64) {
        let now: DateTime<Local> = Local::now();
        let end_date = now.checked_add_days(Days::new(days)).unwrap();
        let mut iter_date = now;
        while iter_date < end_date {
            let availability_block = AvailabilityBlock {
                start: NaiveTime::from_hms_opt(8, 0, 0).unwrap(),
                end: NaiveTime::from_hms_opt(4, 30, 0).unwrap(),
            };
            let mut capacity = Capacity {
                date: iter_date.date_naive(),
                availability: Vec::new(),
            };
            capacity.availability.push(availability_block);
            self.capacity.push(capacity);
            iter_date = iter_date + Duration::days(1);
        }
    }

    fn get_soonest_capacity(&self) -> Option<&AvailabilityBlock> {
        // Just getting the first capacity day for now
        let today = Utc::today().naive_utc();
        let soonest_availability = self.capacity.iter().find(|r| r.date == today);
        let first_avail = soonest_availability.unwrap().get_first_availability();
        first_avail
    }
}

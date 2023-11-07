use chrono::{DateTime, Days, Duration, Local, NaiveDate, NaiveDateTime, NaiveTime};

#[allow(dead_code)]
#[derive(Debug)]
struct Resource {
    id: String,
    resource_type: ResourceType,
    capacity: Vec<Capacity>,
}

impl Resource {
    fn new(id: String, resource_type: ResourceType) -> Resource {
        Resource {
            id,
            resource_type,
            capacity: Vec::new(),
        }
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
                date: iter_date.date().naive_local(),
                availability: Vec::new(),
            };
            capacity.availability.push(availability_block);
            self.capacity.push(capacity);
            iter_date = iter_date + Duration::days(1);
        }
    }

    fn get_soonest_capacity(&self) -> Option<&AvailabilityBlock> {
        // Just getting the first capacity day for now
        let soonest_availability = self.capacity.get(0);
        let first_avail = soonest_availability.unwrap().get_first_availability();
        return first_avail;
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Capacity {
    date: NaiveDate,
    availability: Vec<AvailabilityBlock>,
}

impl Capacity {
    fn get_first_availability(&self) -> Option<&AvailabilityBlock> {
        return self.availability.get(0);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct AvailabilityBlock {
    start: NaiveTime,
    end: NaiveTime,
}

#[allow(dead_code)]
#[derive(Debug)]
enum ResourceType {
    Person,
    Machine,
    Pallet,
}
#[allow(dead_code)]
#[derive(Debug)]
struct Block {
    seq: u32,
    length: f32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
    resources: Vec<Resource>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Operation {
    seq: u32,
    code: String,
    resource: Resource,
    length: i64, // In minutes
    blocks: Vec<Block>,
}

#[allow(dead_code, unused_mut, unused_variables)]
impl Operation {
    fn schedule_op(&self) {
        let mut blocks: Vec<Block> = vec![];
        let operation_length = Duration::minutes(self.length.to_owned());
        println!("{:?}", operation_length);
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct Project {
    id: String,
    description: String,
    blocks: Vec<Block>,
}

#[allow(dead_code)]
struct Job {
    job_num: String,
    need_date: NaiveDate,
    ops: Vec<Operation>,
}
#[allow(dead_code)]
impl Job {
    fn new(job_num: String, need_date: NaiveDate) -> Job {
        Job {
            job_num,
            need_date,
            ops: vec![],
        }
    }

    fn add_operations(&mut self, operations: Vec<Operation>) {
        operations.into_iter().for_each(|op| self.ops.push(op))
    }

    fn schedule_job(&mut self) {
        self.ops.iter().for_each(|op| op.schedule_op())
    }
}

fn main() {
    let mut ycm1 = Resource::new("YCM01".to_string(), ResourceType::Machine);
    let mut ycm2 = Resource::new("YCM02".to_string(), ResourceType::Machine);
    Resource::calculate_capacity(&mut ycm1, 10);
    Resource::calculate_capacity(&mut ycm2, 10);
    println!("{:#?}", ycm2);
    let mut job1 = Job::new(
        "TEST01".to_string(),
        NaiveDate::from_ymd_opt(2023, 11, 10).unwrap(),
    );
    let op1 = Operation {
        seq: 1,
        code: "VERT".to_string(),
        resource: ycm2,
        length: 12 * 60,
        blocks: Vec::new(),
    };
    job1.add_operations(vec![op1]);
    println!("{:?}", job1.need_date);
    job1.schedule_job();
}

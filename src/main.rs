use chrono::{DateTime, Days, Duration, Local, NaiveDate, NaiveTime};
use std::io::stdin;

#[allow(dead_code)]
#[derive(Debug)]
struct Resource {
    id: String,
    resource_type: ResourceType,
    capacity: Vec<Capacity>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Capacity {
    date: NaiveDate,
    availability: Vec<AvailabilityBlock>,
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
    resources: Vec<Resource>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Operation {
    code: String,
    // Need to account for multiple resources at some point
    resource: Resource,
    length: f32,
    blocks: Vec<Block>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Project {
    id: String,
    description: String,
    blocks: Vec<Block>,
}

fn main() {
    let mut vert = Resource {
        id: "VERT".to_string(),
        resource_type: ResourceType::Machine,
        capacity: Vec::new(),
    };
    let now: DateTime<Local> = Local::now();
    let end_date = now.checked_add_days(Days::new(5)).unwrap();
    let mut iter_date = now;
    while iter_date < end_date {
        let mut availability_block = AvailabilityBlock {
            start: NaiveTime::from_hms(8, 0, 0),
            end: NaiveTime::from_hms(4, 30, 0),
        };
        let mut capacity = Capacity {
            date: iter_date.date().naive_local(),
            availability: Vec::new(),
        };
        capacity.availability.push(availability_block);
        vert.capacity.push(capacity);
        iter_date = iter_date + Duration::days(1);
    }
    let op1 = Operation {
        code: "VERT".to_string(),
        resource: vert,
        length: 50.0,
        blocks: Vec::new(),
    };
    // For the operation, find the resource assignment and find the soonest availability.

    println!("Enter a start time for the Verts");

    let mut start_time = String::from("");
    stdin()
        .read_line(&mut start_time)
        .expect("Failed to read start time");
}

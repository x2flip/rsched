use chrono::{DateTime, Duration, Local, NaiveDate, NaiveTime};
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
    let vert = Resource {
        id: "VERT".to_string(),
        resource_type: ResourceType::Machine,
        capacity: Vec::new(),
    };
    let op1 = Operation {
        code: "VERT".to_string(),
        resource: vert,
        length: 50.0,
        blocks: Vec::new(),
    };
    while op1.length > 0.0 {
        unimplemented!();
        // Find the soonest open capacity at the resource group
    }
    println!("Enter a start time for the Verts");

    let mut start_time = String::from("");
    stdin()
        .read_line(&mut start_time)
        .expect("Failed to read start time");

    let now: DateTime<Local> = Local::now();
}

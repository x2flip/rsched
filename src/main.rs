use chrono::{DateTime, Duration, Local};
use std::io::stdin;

#[derive(Debug)]
struct Resource {
    id: String,
    resource_type: ResourceType,
}

#[derive(Debug)]
enum ResourceType {
    Person,
    Machine,
    Pallet,
}

#[derive(Debug)]
struct Block {
    id: String,
    seq: u32,
    length: f32,
    resources: Vec<Resource>,
}
struct Project {
    id: String,
    description: String,
    blocks: Vec<Block>,
}
fn main() {
    let mut project1 = Project {
        id: "project1".to_string(),
        description: "Project 1".to_string(),
        blocks: Vec::new(),
    };
    let mut block1 = Block {
        id: "QUOTE".to_string(),
        seq: 1,
        length: 5.5,
        resources: Vec::new(),
    };
    let mut block2 = Block {
        id: "REVIEW".to_string(),
        seq: 2,
        length: 5.5,
        resources: Vec::new(),
    };
    let mut block3 = Block {
        id: "BUILD".to_string(),
        seq: 3,
        length: 5.5,
        resources: Vec::new(),
    };

    let now: DateTime<Local> = Local::now();
    project1.blocks.push(block1);
    project1.blocks.push(block2);
    project1.blocks.push(block3);
    println!("Here is your project: {}", project1.description);
    println!("Here are your steps: {:#?}", project1.blocks);
    println!("Which step would you like to schedule?");
    project1
        .blocks
        .iter()
        .for_each(|block| println!("{}", block.id));

    println!(
        "Enter a number corresponding sequence number between {} and {}",
        project1.blocks.first().unwrap().seq,
        project1.blocks.last().unwrap().seq
    );
    let mut block_id = String::new();
    stdin().read_line(&mut block_id).unwrap();
    println!("When do you want this operation to start?");

    let mut scheduled_start = String::new();
    stdin().read_line(&mut scheduled_start).unwrap();
    let selected_block = project1
        .blocks
        .iter()
        .find(|&x| x.seq == block_id.trim().parse::<u32>().unwrap())
        .unwrap();
    let duration = Duration::seconds((selected_block.length * 3600.0) as i64);
    let mut end_time = now;
    end_time += duration;
    println!("Start Time: {}", now);
    println!("End Time: {}", end_time);
}

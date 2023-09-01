use chrono::{DateTime, Local};

struct Resource {
    id: String,
    resource_type: ResourceType,
}
enum ResourceType {
    Person,
    Machine,
    Pallet,
}
struct Block {
    id: String,
    length: f32,
    resources: Vec<Resource>,
}
struct Project {
    id: String,
    description: String,
    blocks: Vec<Block>,
}
fn main() {
    let project1 = Project {
        id: "project1".to_string(),
        description: "Project 1".to_string(),
        blocks: Vec::new(),
    };
    let block1 = Block {
        id: "QUOTE".to_string(),
        length: 5.5,
        resources: Vec::new(),
    };
    let block2 = Block {
        id: "QUOTE".to_string(),
        length: 5.5,
        resources: Vec::new(),
    };
    let block3 = Block {
        id: "QUOTE".to_string(),
        length: 5.5,
        resources: Vec::new(),
    };
    let local: DateTime<Local> = Local::now();

    println!("Hello, world!");
}

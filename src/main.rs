use chrono::NaiveDate;

mod capacity;
mod job;
mod load;
mod operation;
mod resource;

fn main() {
    // Create the job
    let mut job = job::Job::create(
        "TEST01".to_string(),
        NaiveDate::from_ymd_opt(2024, 2, 29).unwrap(),
    );

    // Create Resource
    let vert01 = resource::Resource::create(
        "VERT01".to_string(),
        resource::ResourceType::Machine,
        Some(1),
    );

    // Add an operation
    job.add_operation("VERT".to_string(), vert01, 60);

    job.schedule_job();
    println!("{:#?}", job);
}

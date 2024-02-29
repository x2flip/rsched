use chrono::NaiveDate;

mod capacity;
mod job;
mod load;
mod operation;
mod resource;

fn main() {
    // Create the job
    let job_num = "TEST01".to_string();
    let need_date = NaiveDate::from_ymd_opt(2024, 2, 29).unwrap();
    let job = job::Job::create(job_num, need_date);

    println!("{:?}", job);

    // Add an operation
    job.add_operation("VERT".to_string(), resource, length)
}

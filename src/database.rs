use std::fs;
use std::fs::File;
use std::io::{Error, Write};
use chrono::{Local};
use crate::models;

fn read_file() -> Result<Vec<models::Task>, Error> {
    let string_data: String = fs::read_to_string("database.json")?;
    let task_list: Vec<models::Task> = serde_json::from_str(&string_data)?;
    Ok(task_list)
}

fn create_database_for_task(task: models::Task) -> Result<(), Error> {
    let mut file = File::create("database.json")?;

    let task_list = vec![task];
    let serialized_list = serde_json::to_string(&task_list)?;

    file.write_all(serialized_list.as_bytes())?;
    Ok(())
}

fn update_database(task_list: &Vec<models::Task>) -> Result<(), Error> {
    let mut file = File::create("database.json")?;
    let task_data = serde_json::to_string(&task_list)?;
    file.write_all(task_data.as_bytes())
}

pub fn create_task(description: &String) -> Result<(), Error> {
    match read_file() {
        Ok(mut task_list) => {
            let task = models::Task {
                id: (task_list.iter().count() as u32)+1,
                description: description.clone(),
                status: models::Status::Todo,
                created_at: Local::now().to_utc(),
                updated_at: Local::now().to_utc()
            };

            task_list.push(task);
            update_database(&task_list)?;
            Ok(())
        }
        Err(error) => {
            print!("An error occurred while trying to add a task: {error}\nWe will create the database instead");
            let task = models::Task {
                id: 1,
                description: description.clone(),
                status: models::Status::Todo,
                created_at: Local::now().to_utc(),
                updated_at: Local::now().to_utc()
            };

            create_database_for_task(task)
        }
    }
}


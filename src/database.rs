use crate::models::{Status, Task, TaskList};
use std::fs;
use std::fs::File;
use std::io::{Error, Write};

fn read_file() -> Result<TaskList, Error> {
    let string_data: String = fs::read_to_string("database.json")?;
    let task_list: TaskList = serde_json::from_str(&string_data)?;
    Ok(task_list)
}

fn create_database_for_task(task_description: String) -> Result<(), Error> {
    let mut file = File::create("database.json")?;

    let mut task_list = TaskList::new();
    task_list.add(task_description);

    let serialized_list = serde_json::to_string(&task_list)?;

    file.write_all(serialized_list.as_bytes())?;
    Ok(())
}

fn update_database(task_list: &TaskList) -> Result<(), Error> {
    let mut file = File::create("database.json")?;
    let task_data = serde_json::to_string(&task_list)?;
    file.write_all(task_data.as_bytes())
}

pub fn create_task(description: &str) -> Result<(), Error> {
    match read_file() {
        Ok(mut task_list) => {
            task_list.add(description.to_string());
            update_database(&task_list)?;
            Ok(())
        }
        Err(error) => {
            print!(
                "An error occurred while trying to add a task: {error}\nWe will create the database instead"
            );

            create_database_for_task(description.to_string())
        }
    }
}

pub fn read_task_list() -> Result<TaskList, Error> {
    read_file()
}

pub fn task_list_with_status(status: Option<Status>) -> Result<Vec<Task>, Error> {
    let task_list = read_file()?;
    match status {
        Some(status) => {
            Ok(task_list.with_status(status))
        }
        None => Ok(task_list.all_tasks()),
    }
}

pub fn update_task(task: Task) -> Result<(), Error> {
    let mut task_list = read_file()?;
    task_list.update(task);

    update_database(&task_list)
}

pub fn delete_task(id: u32) -> Result<(), Error> {
    let mut task_list = read_file()?;
    task_list.remove(id);
    update_database(&task_list)?;
    Ok(())
}

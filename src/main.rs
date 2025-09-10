mod database;
pub mod models;

use std::env;
use comfy_table::Table;

fn main() {
    let args: Vec<String> = env::args().collect();
    let action_string = args.get(1);

    if let Some(action_string) = action_string {
        if let Some(action) = models::Action::action_from_str(action_string) {
            handle_action(action);
        } else {
            println!("No action found");
        }
    } else {
        println!("No action specified");
    }
}

fn handle_action(action: models::Action) {
    match action {
        models::Action::Add => add_task(),
        models::Action::Update => update_task(),
        models::Action::Delete => delete_task(),
        models::Action::List => list_task(),
        models::Action::Mark(status) => mark_as(status),
    }
}

fn add_task() {
    let description = env::args().nth(2).expect("Please provide a description");
    match database::create_task(&description) {
        Ok(()) => println!("Task {description} created"),
        Err(e) => println!("Task {description} could not be created: {}", e),
    }
}

fn list_task() {
    let status = env::args().nth(2);
    match status {
        Some(status_string) => {
            let status = models::Status::status_from_str(&status_string)
                .expect("Please provide a valid status");
            let task_list  = database::read_task_list(Some(status)).expect("Unable to read list of tasks");
            if task_list.is_empty() {
                println!("No tasks found");
            } else {
                present_task_table(task_list);
            }
        },
        None => {
            let task_list = database::read_task_list(None).expect("Unable to read list of tasks");
            if task_list.is_empty() {
                println!("No tasks found");
            } else {
                present_task_table(task_list)
            }
        }
    }
}

fn present_task_table(task_list: Vec<models::Task>) {
    let mut table = Table::new();

    table.set_header(vec!["Id", "Description", "Status"]);
    for task in task_list {
        table.add_row(vec![task.id.to_string(), task.description, task.status.to_string()]);
    }

    println!("{table}");
}

fn update_task() {}

fn delete_task() {}

fn mark_as(status: models::Status) {
    print!("Mark task as {}", status);
}

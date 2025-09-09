mod database;
pub mod models;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let action_string = args.get(1);

    if let Some(action_string) = action_string {
        if let Some(action) = retrieve_action(action_string) {
            handle_action(action);
        } else {
            println!("No action found");
        }
    } else {
        println!("No action specified");
    }
}

fn retrieve_action(string: &String) -> Option<models::Action> {
    match string.as_ref() {
        "add" => Some(models::Action::Add),
        "update" => Some(models::Action::Update),
        "delete" => Some(models::Action::Delete),
        "mark-to-do" => Some(models::Action::Mark(models::Status::Todo)),
        "mark-in-progress" => Some(models::Action::Mark(models::Status::InProgress)),
        "mark-done" => Some(models::Action::Mark(models::Status::Done)),
        _ => None,
    }
}

fn handle_action(action: models::Action) {
    match action {
        models::Action::Add => add_task(),
        models::Action::Update => update_task(),
        models::Action::Delete => delete_task(),
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

fn update_task() {}

fn delete_task() {}

fn mark_as(status: models::Status) {
    print!("Mark task as {}", status);
}

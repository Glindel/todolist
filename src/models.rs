use chrono::{DateTime, Utc, Local};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::mem::replace;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    id: u32,
    description: String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl Task {
    pub fn new(id: u32, description: String) -> Self {
        Task {
            id,
            description,
            status: Status::Todo,
            created_at: Local::now().to_utc(),
            updated_at: Local::now().to_utc()
        }
    }

    pub fn id(&self) -> u32 {
        self.id
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &Status {
        &self.status
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }

    pub fn updated_at(&self) -> &DateTime<Utc> {
        &self.updated_at
    }

    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    pub fn set_status(&mut self, status: Status) {
        self.status = status;
    }

    pub fn set_updated_at(&mut self, date: DateTime<Utc>) {
        self.updated_at = date;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TaskList {
    task_list: Vec<Task>,
    next_id: u32
}

impl TaskList {
    pub fn new() -> Self {
        TaskList { task_list: vec![], next_id: 1 }
    }

    pub fn add(&mut self, description: String) {
        self.task_list.push(Task::new(self.next_id, description));
        self.next_id = self.next_id + 1; 
    }

    pub fn get(&self, id: u32) -> Option<&Task> {
        let index = (id - 1) as usize;
        self.task_list.get(index)
    }

    pub fn update(&mut self, task: Task) {
        let index = self.task_list.iter().position(|t| t.id() == task.id()).expect("The task doesn't exist in the list");
        let _ = replace(&mut self.task_list[index], task);
    }

    pub fn remove(&mut self, id: u32) {
        let index = (id - 1) as usize;
        self.task_list.remove(index);
    }

    pub fn all_tasks(&self) -> Vec<Task> {
        self.task_list.clone()
    }

    pub fn with_status(&self, status: Status) -> Vec<Task> {
        self.task_list
        .clone()
        .into_iter()
        .filter(|task| *task.status() == status)
        .collect()
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Copy, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    pub fn status_from_str(string: &str) -> Option<Status> {
        match string {
            "todo" => Some(Status::Todo),
            "in-progress" => Some(Status::InProgress),
            "done" => Some(Status::Done),
            _ => None
        }
    }
}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Status::Todo => write!(f, "todo"),
            Status::InProgress => write!(f, "in-progress"),
            Status::Done => write!(f, "done"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Action {
    Add,
    Update,
    Delete,
    List,
    Mark(Status)
}

impl Action {
    pub fn action_from_str(string: &str) -> Option<Action> {
        match string.as_ref() {
            "add" => Some(Action::Add),
            "update" => Some(Action::Update),
            "delete" => Some(Action::Delete),
            "list" => Some(Action::List),
            "mark-to-do" => Some(Action::Mark(Status::Todo)),
            "mark-in-progress" => Some(Action::Mark(Status::InProgress)),
            "mark-done" => Some(Action::Mark(Status::Done)),
            _ => None,
        }
    }
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Add => write!(f, "add"),
            Action::Update => write!(f, "update"),
            Action::Delete => write!(f, "delete"),
            Action::List => write!(f, "list"),
            Action::Mark(status) => write!(f, "mark-{}", status)
        }
    }
}

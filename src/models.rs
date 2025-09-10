use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Task {
    pub fn index(&self) -> usize {
        (self.id as usize) - 1
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

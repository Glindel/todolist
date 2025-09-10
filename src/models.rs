use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use crate::models;

#[derive(Serialize, Deserialize, Debug)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}

impl Status {
    pub fn status_from_str(string: &String) -> Option<Status> {
        match string.as_ref() {
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

#[derive(PartialEq, Eq)]
pub enum Action {
    Add,
    Update,
    Delete,
    List,
    Mark(Status)
}

impl Action {
    pub fn action_from_str(string: &String) -> Option<Action> {
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

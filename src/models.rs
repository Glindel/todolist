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

#[derive(Serialize, Deserialize, Debug)]
pub enum Status {
    Todo,
    InProgress,
    Done,
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

pub enum Action {
    Add,
    Update,
    Delete,
    Mark(Status)
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Action::Add => write!(f, "add"),
            Action::Update => write!(f, "update"),
            Action::Delete => write!(f, "delete"),
            Action::Mark(status) => write!(f, "mark-{}", status)
        }
    }
}

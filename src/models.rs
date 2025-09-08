use chrono::{DateTime, TimeZone, Utc, serde::ts_seconds};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    id:i32,
    description:String,
    status: Status,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>
}

#[derive(Serialize, Deserialize, Debug)]
enum Status {
    Todo,
    InProgress,
    Done
}

use chrono::prelude::*;

#[derive(Clone)]
pub struct Hope {
    pub id: i32,
    pub title: String,
    pub deadline: NaiveDate,
}

#[derive(Clone)]
pub struct Process {
    pub id: i32,
    pub title: String,
    pub hope_id: i32,
}

#[derive(Clone)]
pub struct Task {
    pub id: i32,
    pub title: String,
    pub input: String,
    pub action: String,
    pub output: String,
    pub weight: i32,
    pub process_id: Option<i32>,
    pub is_done: bool,
}

#[derive(Clone)]
pub struct HopeBlock {
    pub hope: Hope,
    pub process: Vec<ProcessBlock>,
}

#[derive(Clone)]
pub struct ProcessBlock {
    pub process: Process,
    pub tasks: Vec<Task>,
}

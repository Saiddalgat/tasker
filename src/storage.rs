use crate::tasks::Task;
use std::fs;
use std::path::Path;

const FILE: &str = "tasks.json";

pub fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE).exists() {
        return Vec::new();
    }
    let data = fs::read_to_string(FILE).unwrap_or_else(|_| "[]".to_string());
    serde_json::from_str(&data).unwrap_or_else(|_| Vec::new())
}

pub fn save_tasks(tasks: &Vec<Task>) {
    let data = serde_json::to_string_pretty(tasks).unwrap();
    fs::write(FILE, data).expect("Failed to write tasks file");
}

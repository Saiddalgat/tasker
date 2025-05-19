use crate::storage;

pub fn handle_list() {
    let tasks = storage::load_tasks();
    for (i, task) in tasks.iter().enumerate() {
        println!("{}. [{}] {}", i + 1, if task.done { "x" } else { " " }, task.description);
    }
}

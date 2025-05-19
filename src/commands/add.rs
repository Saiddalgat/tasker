use crate::{storage, tasks::Task};

pub fn handle_add(description: &str) {
    let mut tasks = storage::load_tasks();
    let new_task = Task::new(description.to_string());
    tasks.push(new_task);
    storage::save_tasks(&tasks);
    println!("✅ Задача добавлена!");
}

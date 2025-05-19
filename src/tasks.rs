use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub description: String,
    pub done: bool,
}

impl Task {
    pub fn new(description: String) -> Self {
        Task {
            description,
            done: false,
        }
    }
}

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    description: String,
    done: bool,
}

impl Task {
    fn new(desc: String) -> Self {
        Task {
            description: desc,
            done: false,
        }
    }
}

#[derive(Default)]
struct TaskApp {
    tasks: Vec<Task>,
    new_task: String,
}

impl TaskApp {
    fn load_tasks(&mut self) {
        self.tasks = fs::read_to_string("tasks.json")
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default();
    }

    fn save_tasks(&self) {
        let data = serde_json::to_string_pretty(&self.tasks).unwrap();
        fs::write("tasks.json", data).expect("Failed to save tasks");
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📋 Tasker GUI");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                if ui.button("Добавить").clicked() {
                    if !self.new_task.trim().is_empty() {
                        self.tasks.push(Task::new(self.new_task.trim().to_string()));
                        self.new_task.clear();
                        self.save_tasks();
                    }
                }
            });

            ui.separator();

            let mut changed_any = false;

            for task in &mut self.tasks {
                ui.horizontal(|ui| {
                    let changed = ui.checkbox(&mut task.done, "").changed();
                    ui.label(&task.description);

                    if changed {
                        changed_any = true;
                    }
                });
            }

            if changed_any {
                self.save_tasks();
            }

            if ui.button("💾 Сохранить").clicked() {
                self.save_tasks();
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut app = TaskApp::default();
    app.load_tasks();

    let options = eframe::NativeOptions::default();
    eframe::run_native("Tasker GUI", options, Box::new(|_cc| Box::new(app)))
}
//GUI version
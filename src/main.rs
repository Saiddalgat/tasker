use chrono::{Datelike, Local, NaiveDate};
use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Clone)]
struct Task {
    description: String,
    done: bool,
    deadline: Option<String>,
    category: String,
}

impl Task {
    fn new(desc: String, deadline: Option<String>, category: String) -> Self {
        Task {
            description: desc,
            done: false,
            deadline,
            category,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct AppSettings {
    dark_mode: bool,
}

#[derive(Default)]
struct TaskApp {
    tasks: Vec<Task>,
    new_task: String,
    new_deadline: String,
    new_category: usize,
    dark_mode: bool,
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

    fn load_settings(&mut self) {
        if let Ok(data) = fs::read_to_string("settings.json") {
            if let Ok(settings) = serde_json::from_str::<AppSettings>(&data) {
                self.dark_mode = settings.dark_mode;
            }
        }
    }

    fn save_settings(&self) {
        let data = serde_json::to_string_pretty(&AppSettings {
            dark_mode: self.dark_mode,
        })
        .unwrap();
        fs::write("settings.json", data).expect("Failed to save settings");
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // применяем тему
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        let categories = vec!["Личное", "Работа", "Учёба", "Проект", "Другое"];

        // Переключатель темы
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🌗 Тема:");
                let theme_label = if self.dark_mode { "Тёмная" } else { "Светлая" };
                if ui.checkbox(&mut self.dark_mode, theme_label).changed() {
                    self.save_settings();
                }
            });
        });

        // Главная панель
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📋 Tasker GUI");

            // Новая задача
            ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_task);
                ui.label("до:");
                ui.text_edit_singleline(&mut self.new_deadline);
            });

            ui.horizontal(|ui| {
                egui::ComboBox::from_label("Категория")
                    .selected_text(categories[self.new_category])
                    .show_ui(ui, |ui| {
                        for (i, cat) in categories.iter().enumerate() {
                            ui.selectable_value(&mut self.new_category, i, *cat);
                        }
                    });

                if ui.button("Добавить").clicked() {
                    if !self.new_task.trim().is_empty() {
                        let deadline = if self.new_deadline.trim().is_empty() {
                            None
                        } else {
                            Some(self.new_deadline.trim().to_string())
                        };

                        self.tasks.push(Task::new(
                            self.new_task.trim().to_string(),
                            deadline,
                            categories[self.new_category].to_string(),
                        ));

                        self.new_task.clear();
                        self.new_deadline.clear();
                        self.new_category = 0;
                        self.save_tasks();
                    }
                }
            });

            ui.separator();

            let mut changed_any = false;
            let mut to_remove: Option<usize> = None;
            let today = Local::now().naive_local().date();

            for (i, task) in self.tasks.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    let changed = ui.checkbox(&mut task.done, "").changed();

                    let mut label = if task.done {
                        egui::RichText::new(&task.description)
                            .strikethrough()
                            .italics()
                            .weak()
                    } else {
                        egui::RichText::new(&task.description)
                    };

                    // Подсветка просроченных
                    if !task.done {
                        if let Some(date_str) = &task.deadline {
                            if let Ok(date) = NaiveDate::parse_from_str(date_str, "%d.%m.%Y") {
                                if date < today {
                                    label = label.color(egui::Color32::RED);
                                }
                            }
                        }
                    }

                    ui.label(label);

                    if let Some(d) = &task.deadline {
                        ui.label(format!("⏰ до {}", d));
                    }

                    ui.label(format!("📁 {}", task.category));

                    if ui.button("🗑").clicked() {
                        to_remove = Some(i);
                    }

                    if changed {
                        changed_any = true;
                    }
                });
            }

            if changed_any {
                self.save_tasks();
            }

            if let Some(index) = to_remove {
                self.tasks.remove(index);
                self.save_tasks();
            }

            ui.separator();
            if ui.button("💾 Сохранить").clicked() {
                self.save_tasks();
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let mut app = TaskApp::default();
    app.load_tasks();
    app.load_settings();

    let options = eframe::NativeOptions::default();
    eframe::run_native("Tasker GUI", options, Box::new(|_cc| Box::new(app)))
}

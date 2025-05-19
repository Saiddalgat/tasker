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

fn category_color(category: &str) -> egui::Color32 {
    match category {
        "Работа" => egui::Color32::from_rgb(70, 130, 180),
        "Учёба" => egui::Color32::from_rgb(123, 104, 238),
        "Проект" => egui::Color32::from_rgb(255, 165, 0),
        "Личное" => egui::Color32::from_rgb(46, 204, 113),
        "Другое" => egui::Color32::LIGHT_GRAY,
        _ => egui::Color32::GRAY,
    }
}

impl eframe::App for TaskApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.dark_mode {
            ctx.set_visuals(egui::Visuals::dark());
        } else {
            ctx.set_visuals(egui::Visuals::light());
        }

        let categories = vec!["Личное", "Работа", "Учёба", "Проект", "Другое"];

        egui::TopBottomPanel::top("top_theme").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("🌗 Тема:");
                let theme_label = if self.dark_mode { "Тёмная" } else { "Светлая" };
                if ui.checkbox(&mut self.dark_mode, theme_label).changed() {
                    self.save_settings();
                }
            });
        });

        let total = self.tasks.len();
        let done = self.tasks.iter().filter(|t| t.done).count();
        egui::TopBottomPanel::top("top_progress").show(ctx, |ui| {
            if total > 0 {
                let percent = if total == 0 { 0.0 } else { done as f32 / total as f32 };
                let color = if percent < 0.3 {
                    egui::Color32::RED
                } else if percent < 0.7 {
                    egui::Color32::YELLOW
                } else {
                    egui::Color32::GREEN
                };
                ui.label(format!("✅ Выполнено: {}/{} ({:.0}%)", done, total, percent * 100.0));
               ui.add(
                    egui::ProgressBar::new(percent)
                        .fill(color)
                        .desired_width(f32::INFINITY)
                        .desired_height(10.0) // <--- фикс!
                );
            } else {
                ui.label("📝 Задач пока нет");
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📋 Tasker GUI");

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
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        let changed = ui.checkbox(&mut task.done, "").changed();

                        let mut label = if task.done {
                            egui::RichText::new(&task.description)
                                .strikethrough()
                                .italics()
                                .weak()
                        } else {
                            egui::RichText::new(&task.description).strong()
                        };

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

                        let color = category_color(&task.category);
                        ui.colored_label(color, format!("⬤ {}", task.category));

                        if ui.button("🗑").clicked() {
                            to_remove = Some(i);
                        }

                        if changed {
                            changed_any = true;
                        }
                    });
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn task_creation_works() {
        let t = Task::new("Протестировать".to_string(), Some("01.01.2025".to_string()), "Тест".to_string());
        assert_eq!(t.description, "Протестировать");
        assert_eq!(t.done, false);
        assert_eq!(t.deadline, Some("01.01.2025".to_string()));
        assert_eq!(t.category, "Тест");
    }
}
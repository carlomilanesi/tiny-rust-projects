#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::{egui, epaint::Vec2, Theme};

fn main() {
    eframe::run_native(
        "Adder",
        eframe::NativeOptions {
            initial_window_size: Some(Vec2::new(250., 100.)),
            resizable: false,
            default_theme: Theme::Light,
            ..Default::default()
        },
        Box::new(|_| {
            Box::new(AdderState {
                addend1: "".to_string(),
                addend2: "".to_string(),
                sum: "".to_string(),
            })
        }),
    );
}

struct AdderState {
    addend1: String,
    addend2: String,
    sum: String,
}

impl AdderState {
    fn compute_sum(&mut self) {
        self.sum =
            if let (Ok(a1), Ok(a2)) = (self.addend1.parse::<u64>(), self.addend2.parse::<u64>()) {
                if let Some(s) = a1.checked_add(a2) {
                    s.to_string()
                } else {
                    "---".to_string()
                }
            } else {
                "---".to_string()
            };
    }
}

impl eframe::App for AdderState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.compute_sum();
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Grid::new("main_grid").num_columns(2).show(ui, |ui| {
                ui.label("Addend 1: ");
                ui.text_edit_singleline(&mut self.addend1);
                ui.end_row();

                ui.label("Addend 2: ");
                ui.text_edit_singleline(&mut self.addend2);
                ui.end_row();

                ui.label("Sum: ");
                ui.colored_label(egui::Color32::from_rgb(128, 0, 128), &self.sum);
                ui.end_row();
            });
        });
    }
}

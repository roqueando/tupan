pub mod commands;
pub mod persistence;
pub mod state;

use crate::app::state::AppState;
use eframe::egui;

/// Main application struct for Tupan.
pub struct TupanApp {
    /// Engineering workspace state
    pub state: AppState,
}

impl TupanApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            state: AppState::default(),
        }
    }
}

impl eframe::App for TupanApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Apply theme
        if self.state.theme == state::Theme::Dark {
            ui.visuals_mut().dark_mode = true;
        } else {
            ui.visuals_mut().dark_mode = false;
        }

        // ===== MINIMAL TOOLBAR =====
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ Tupan — Component Canvas");
                ui.separator();
                ui.label(&self.state.status_message);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Theme toggle
                    if ui.button("🌙/☀️").clicked() {
                        self.state.theme = match self.state.theme {
                            state::Theme::Dark => state::Theme::Light,
                            state::Theme::Light => state::Theme::Dark,
                        };
                    }
                });
            });
        });

        // ===== MAIN CONTENT =====
        egui::CentralPanel::default().show_inside(ui, |ui| {
            crate::ui::component_canvas::show_component_canvas(ui, &mut self.state);
        });
    }
}

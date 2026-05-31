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
        let is_dark = self.state.theme == state::Theme::Dark;
        if is_dark { ui.visuals_mut().dark_mode = true; } else { ui.visuals_mut().dark_mode = false; }

        let tb_bg = if is_dark { egui::Color32::from_rgb(22, 22, 32) } else { egui::Color32::from_rgb(255, 255, 255) };
        let tb_fg = if is_dark { egui::Color32::from_rgb(220, 225, 240) } else { egui::Color32::from_rgb(30, 35, 50) };
        let tb_sec = if is_dark { egui::Color32::from_rgb(140, 150, 175) } else { egui::Color32::from_rgb(110, 115, 135) };
        let accent = egui::Color32::from_rgb(99, 130, 255);

        // ── Toolbar ──
        egui::Panel::top("toolbar")
            .frame(egui::Frame { fill: tb_bg, inner_margin: egui::Margin::symmetric(16, 8), ..Default::default() })
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("⚡").color(accent).size(20.0));
                    ui.label(egui::RichText::new("Tupan").color(tb_fg).size(16.0).strong());
                    ui.separator();
                    ui.label(egui::RichText::new(&self.state.status_message).color(tb_sec).size(12.0));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let btn = if is_dark { "☀️" } else { "🌙" };
                        if ui.button(btn).clicked() {
                            self.state.theme = match self.state.theme {
                                state::Theme::Dark => state::Theme::Light,
                                state::Theme::Light => state::Theme::Dark,
                            };
                        }
                    });
                });
            });

        // ── Main content ──
        egui::CentralPanel::default().show_inside(ui, |ui| {
            crate::ui::component_canvas::show_component_canvas(ui, &mut self.state);
        });
    }
}

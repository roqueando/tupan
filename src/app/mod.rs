pub mod commands;
pub mod persistence;
pub mod state;

use crate::app::state::{AppState, AppTab};
use crate::ui::schematic_editor::show_schematic_editor;
use crate::ui::workspace::show_workspace;
use eframe::egui;
use std::path::PathBuf;

/// Main application struct for Tupan.
pub struct TupanApp {
    /// Engineering workspace state
    pub state: AppState,

    /// Project file path
    project_path: PathBuf,

    /// Whether a file dialog is pending
    pending_export_svg: bool,
}

impl TupanApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut state = AppState::default();
        state.status_message = "ready — use File menu to save/load".to_owned();
        state.recalculate();

        Self {
            state,
            project_path: PathBuf::from("project.tupan.json"),
            pending_export_svg: false,
        }
    }
}

impl eframe::App for TupanApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Handle deferred SVG export
        if self.pending_export_svg {
            self.pending_export_svg = false;

            let elements = match self.state.active_tab {
                AppTab::Converters => {
                    let comp_values = crate::schematic::layout::ComponentValues {
                        vin: format_eng(self.state.params.vin, "V"),
                        vout: format_eng(self.state.results.vout, "V"),
                        inductance: format_eng(self.state.params.inductance, "H"),
                        capacitance: format_eng(self.state.params.capacitance, "F"),
                        load: format_eng(self.state.params.load_resistance, "Ω"),
                        frequency: format_eng(self.state.params.frequency, "Hz"),
                        _duty_cycle: format!(
                            "{:.1}%",
                            self.state.params.duty_cycle * 100.0
                        ),
                    };
                    crate::schematic::layout::generate_schematic(
                        self.state.active_converter,
                        &comp_values,
                    )
                }
                AppTab::SchematicEditor => self.state.editor.elements.clone(),
            };

            let svg_path = match self.state.active_tab {
                AppTab::Converters => format!("schematic_{}.svg", self.state.active_converter.name()),
                AppTab::SchematicEditor => "schematic_drawing.svg".to_owned(),
            };

            match crate::app::persistence::export_schematic_svg(&svg_path, &elements) {
                Ok(()) => {
                    self.state.status_message = format!("SVG exported to {}", svg_path);
                }
                Err(e) => {
                    self.state.status_message = format!("SVG export failed: {}", e);
                }
            }
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        // Apply theme
        if self.state.theme == state::Theme::Dark {
            ui.visuals_mut().dark_mode = true;
        } else {
            ui.visuals_mut().dark_mode = false;
        }

        // ===== TOOLBAR =====
        egui::Panel::top("toolbar").show_inside(ui, |ui| {
            ui.horizontal(|ui| {
                ui.heading("⚡ Tupan");
                ui.separator();

                // Tab switcher
                let conv_selected = self.state.active_tab == AppTab::Converters;
                if ui
                    .selectable_label(conv_selected, "⚙ Converters")
                    .clicked()
                    && !conv_selected
                {
                    self.state.switch_tab(AppTab::Converters);
                }

                let editor_selected = self.state.active_tab == AppTab::SchematicEditor;
                if ui
                    .selectable_label(editor_selected, "✏️ Schematic Editor")
                    .clicked()
                    && !editor_selected
                {
                    self.state.switch_tab(AppTab::SchematicEditor);
                }

                ui.separator();
                ui.label(&self.state.status_message);

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    // Export SVG button
                    if ui.button("📷 Export SVG").clicked() {
                        self.pending_export_svg = true;
                    }

                    // Save project button
                    if ui.button("💾 Save").clicked() {
                        let path = self.project_path.to_str().unwrap_or("project.tupan.json");
                        match crate::app::persistence::save_project(path, &self.state) {
                            Ok(()) => {
                                self.state.status_message = format!("Saved to {}", path);
                            }
                            Err(e) => {
                                self.state.status_message = format!("Save failed: {}", e);
                            }
                        }
                    }

                    // Load project button
                    if ui.button("📂 Load").clicked() {
                        let path = self.project_path.to_str().unwrap_or("project.tupan.json");
                        match crate::app::persistence::load_project(path) {
                            Ok(loaded_state) => {
                                self.state = loaded_state;
                                self.state.status_message = format!("Loaded from {}", path);
                            }
                            Err(e) => {
                                self.state.status_message = format!("Load failed: {}", e);
                            }
                        }
                    }

                    // Theme toggle
                    if ui.button("🌙/☀️").clicked() {
                        self.state.theme = match self.state.theme {
                            state::Theme::Dark => state::Theme::Light,
                            state::Theme::Light => state::Theme::Dark,
                        };
                    }

                    // Schematic toggle (only relevant for converters tab)
                    if self.state.active_tab == AppTab::Converters {
                        ui.checkbox(&mut self.state.show_schematic, "Schematic");
                    }
                });
            });
        });

        // ===== MAIN CONTENT =====
        egui::CentralPanel::default().show_inside(ui, |ui| {
            match self.state.active_tab {
                AppTab::Converters => show_workspace(ui, &mut self.state),
                AppTab::SchematicEditor => show_schematic_editor(ui, &mut self.state),
            }
        });
    }
}

/// Format a value with SI prefix (duplicated from result_panel for standalone use).
fn format_eng(value: f64, unit: &str) -> String {
    let abs_val = value.abs();
    if abs_val == 0.0 {
        return format!("0 {}", unit);
    }
    let (scaled, prefix) = if abs_val >= 1_000_000.0 {
        (value / 1_000_000.0, "M")
    } else if abs_val >= 1_000.0 {
        (value / 1_000.0, "k")
    } else if abs_val >= 1.0 {
        (value, "")
    } else if abs_val >= 0.001 {
        (value * 1_000.0, "m")
    } else if abs_val >= 0.000_001 {
        (value * 1_000_000.0, "μ")
    } else if abs_val >= 1e-9 {
        (value * 1e9, "n")
    } else {
        (value * 1e12, "p")
    };
    let decimals = if scaled.abs() > 100.0 {
        1
    } else if scaled.abs() > 10.0 {
        2
    } else {
        3
    };
    format!("{:.prec$} {}{}", scaled, prefix, unit, prec = decimals)
}

use crate::app::state::AppState;
use crate::ui::converter_selector::show_converter_selector;
use crate::ui::param_panel::show_param_panel;
use crate::ui::plot_panel::show_plot_panel;
use crate::ui::result_panel::show_result_panel;
use crate::ui::schematic_view::show_schematic;

/// Render the main workspace layout with all panels.
pub fn show_workspace(ui: &mut egui::Ui, state: &mut AppState) {
    // Left panel: parameters
    egui::Panel::left("param_panel")
        .resizable(true)
        .default_size(250.0)
        .min_size(200.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("⚙ Parameters");
                ui.separator();
                show_converter_selector(ui, state);
                ui.add_space(8.0);
                show_param_panel(ui, state);
            });
        });

    // Right panel: results
    egui::Panel::right("result_panel")
        .resizable(true)
        .default_size(220.0)
        .min_size(180.0)
        .show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                show_result_panel(ui, state);
            });
        });

    // Center panel: schematic + plots
    egui::CentralPanel::default().show_inside(ui, |ui| {
        // Vertical split: schematic on top, plots on bottom
        egui::Panel::top("schematic_area")
            .resizable(true)
            .default_size(180.0)
            .min_size(120.0)
            .show_inside(ui, |ui| {
                ui.heading("🔌 Schematic");
                ui.separator();
                show_schematic(ui, state);
            });

        // Bottom: plots
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.heading("📈 Waveforms");
            ui.separator();
            show_plot_panel(ui, state);
        });
    });
}

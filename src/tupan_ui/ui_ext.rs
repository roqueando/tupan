//! Extension trait for [`egui::Ui`] providing Rerun-inspired helper methods.

use crate::tupan_ui::{DesignTokens, design_tokens_for};

/// Extension trait for [`egui::Ui`] with convenience helpers.
pub trait UiExt {
    fn ui(&self) -> &egui::Ui;
    fn ui_mut(&mut self) -> &mut egui::Ui;

    /// Get design tokens for the current theme.
    fn tokens(&self) -> &'static DesignTokens {
        design_tokens_for(self.ui().theme())
    }

    /// Paint a section header label with a separator below it.
    fn section_header(&mut self, text: &str) {
        let tokens = self.tokens();
        self.ui_mut().add_space(2.0);
        self.ui_mut().label(
            egui::RichText::new(text)
                .color(tokens.text_secondary)
                .size(10.0)
                .strong()
                .monospace(),
        );
        self.ui_mut().separator();
    }

    /// Show a result row (label + value) for computed values.
    fn result_row(&mut self, label: &str, value: f64, unit: &str) {
        let tokens = self.tokens();
        self.ui_mut().horizontal(|ui| {
            ui.label(
                egui::RichText::new(label)
                    .color(tokens.text_secondary)
                    .size(11.0)
                    .monospace(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format_eng(value, unit))
                        .color(tokens.text_value)
                        .size(11.0)
                        .monospace(),
                );
            });
        });
    }

    /// Show a param row with a draggable value.
    fn param_row(
        &mut self,
        label: &str,
        value: &mut f64,
        min: f64,
        max: f64,
        speed: f64,
        suffix: &str,
    ) -> bool {
        let tokens = self.tokens();
        self.ui_mut()
            .horizontal(|ui| {
                ui.label(
                    egui::RichText::new(label)
                        .color(tokens.text_secondary)
                        .size(11.0)
                        .monospace(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::DragValue::new(value)
                            .speed(speed)
                            .range(min..=max)
                            .suffix(suffix),
                    )
                    .changed()
                })
                .inner
            })
            .inner
    }

    /// Show a percentage param row.
    fn param_pct(&mut self, label: &str, value: &mut f64, min: f64, max: f64) -> bool {
        let mut display = *value * 100.0;
        let tokens = self.tokens();
        let changed = self
            .ui_mut()
            .horizontal(|ui| {
                ui.label(
                    egui::RichText::new(label)
                        .color(tokens.text_secondary)
                        .size(11.0)
                        .monospace(),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::DragValue::new(&mut display)
                            .speed(0.5)
                            .range(min * 100.0..=max * 100.0)
                            .suffix(" %"),
                    )
                    .changed()
                })
                .inner
            })
            .inner;
        if changed {
            *value = (display / 100.0).clamp(min, max);
        }
        changed
    }
}

impl UiExt for egui::Ui {
    fn ui(&self) -> &egui::Ui {
        self
    }
    fn ui_mut(&mut self) -> &mut egui::Ui {
        self
    }
}

// ── Engineering notation ──────────────────────────────────────────────

fn format_value(value: f64, unit: &str) -> String {
    let av = value.abs();
    if av == 0.0 {
        return format!("0 {}", unit);
    }
    let (scaled, prefix) = if av >= 1_000_000.0 {
        (value / 1_000_000.0, "M")
    } else if av >= 1_000.0 {
        (value / 1_000.0, "k")
    } else if av >= 1.0 {
        (value, "")
    } else if av >= 0.001 {
        (value * 1_000.0, "m")
    } else if av >= 0.000_001 {
        (value * 1_000_000.0, "μ")
    } else if av >= 1e-9 {
        (value * 1e9, "n")
    } else {
        (value * 1e12, "p")
    };
    let dec = if scaled.abs() >= 100.0 {
        1
    } else if scaled.abs() >= 10.0 {
        2
    } else {
        3
    };
    format!("{:.prec$} {}{}", scaled, prefix, unit, prec = dec)
}

fn format_eng(value: f64, unit: &str) -> String {
    format_value(value, unit)
}

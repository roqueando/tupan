use crate::app::state::AppState;
use crate::domain::ConverterType;
use egui::{Color32, RichText, Ui};

/// Display calculated results in the right panel.
pub fn show_result_panel(ui: &mut Ui, state: &mut AppState) {
    let res = &state.results;

    ui.heading("📊 Results");
    ui.separator();

    // Show duty cycle for Buck/Boost
    match state.active_converter {
        ConverterType::Buck | ConverterType::Boost => {
            show_value_row(ui, "Duty cycle", state.params.duty_cycle, "");
        }
        _ => {}
    }
    show_value_row(ui, "Switching freq", state.params.frequency, "Hz");

    ui.add_space(4.0);
    ui.label(RichText::new("Voltage / Current").strong().size(14.0));
    ui.separator();
    show_value_row(ui, "Vout", res.vout, "V");
    show_value_row(ui, "Iout", res.iout, "A");
    show_value_row(ui, "Iin", res.iin, "A");

    // Show VSI-specific results
    if let Some(vrms) = res.rms_output {
        show_value_row(ui, "Vrms", vrms, "V");
    }
    if let Some(v1) = res.fundamental_amplitude {
        show_value_row(ui, "V1 (fund)", v1, "V");
    }

    ui.add_space(8.0);
    ui.label(RichText::new("Ripple").strong().size(14.0));
    ui.separator();
    show_value_row(ui, "V ripple (pp)", res.vout_ripple, "V");
    show_value_row(ui, "I_L ripple (pp)", res.il_ripple, "A");

    ui.add_space(8.0);
    ui.label(RichText::new("Losses & Efficiency").strong().size(14.0));
    ui.separator();
    show_value_row(ui, "Conduction loss", res.conduction_losses, "W");
    show_value_row(ui, "Switching loss", res.switching_losses, "W");
    show_value_row(
        ui,
        "Total loss",
        res.conduction_losses + res.switching_losses,
        "W",
    );

    // Efficiency with color coding
    let eff = res.efficiency;
    let eff_color = if eff > 0.95 {
        Color32::GREEN
    } else if eff > 0.85 {
        Color32::YELLOW
    } else {
        Color32::RED
    };

    ui.horizontal(|ui| {
        ui.label("Efficiency:");
        ui.label(
            RichText::new(format!("{:.1}%", eff * 100.0))
                .color(eff_color)
                .strong()
                .size(16.0),
        );
    });

    // THD for VSI
    if let Some(thd) = res.thd {
        ui.add_space(8.0);
        ui.label(RichText::new("Harmonics").strong().size(14.0));
        ui.separator();
        let thd_color = if thd < 0.5 {
            Color32::GREEN
        } else if thd < 1.0 {
            Color32::YELLOW
        } else {
            Color32::RED
        };
        ui.horizontal(|ui| {
            ui.label("THD:");
            ui.label(
                RichText::new(format!("{:.1}%", thd * 100.0))
                    .color(thd_color)
                    .strong(),
            );
        });
    }

    ui.add_space(12.0);
    ui.separator();
    ui.label(RichText::new("Status").strong().size(14.0));
    ui.label(&state.status_message);
}

fn show_value_row(ui: &mut Ui, label: &str, value: f64, unit: &str) {
    ui.horizontal(|ui| {
        ui.label(format!("{}:", label));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.colored_label(
                egui::Color32::from_rgb(150, 200, 255),
                format_value(value, unit),
            );
        });
    });
}

/// Format a value with appropriate SI prefix and significant digits.
fn format_value(value: f64, unit: &str) -> String {
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

    // Choose decimal places based on magnitude
    let decimals = if scaled.abs() > 100.0 {
        1
    } else if scaled.abs() > 10.0 {
        2
    } else {
        3
    };

    format!("{:.prec$} {}{}", scaled, prefix, unit, prec = decimals)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_value() {
        assert_eq!(format_value(12.0, "V"), "12.00 V");
        assert_eq!(format_value(0.005, "A"), "5.000 mA");
        assert_eq!(format_value(0.000_100, "H"), "100.00 μH");
        assert_eq!(format_value(1000.0, "Hz"), "1.000 kHz");
        assert_eq!(format_value(1_000_000.0, "Hz"), "1.000 MHz");
        assert_eq!(format_value(0.0, "V"), "0 V");
    }
}

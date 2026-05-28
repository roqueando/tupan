use crate::app::state::AppState;
use crate::domain::ConverterType;
use crate::schematic::layout::{generate_schematic, ComponentValues};
use crate::schematic::renderer::draw_element;
use egui::{Color32, Pos2, Ui, Vec2};

/// Render the schematic diagram for the active converter.
pub fn show_schematic(ui: &mut Ui, state: &mut AppState) {
    let (response, painter) = ui.allocate_painter(
        Vec2::new(ui.available_width(), 180.0),
        egui::Sense::hover(),
    );

    if !state.show_schematic {
        ui.label("Schematic panel hidden.");
        return;
    }

    let origin = response.rect.min;

    // Build component values for annotation
    let comp_values = ComponentValues {
        vin: format_eng(state.params.vin, "V"),
        vout: format_eng(state.results.vout, "V"),
        inductance: format_eng(state.params.inductance, "H"),
        capacitance: format_eng(state.params.capacitance, "F"),
        load: format_eng(state.params.load_resistance, "Ω"),
        frequency: format_eng(state.params.frequency, "Hz"),
        _duty_cycle: format!("{:.1}%", state.params.duty_cycle * 100.0),
    };

    let elements = generate_schematic(state.active_converter, &comp_values);

    // Draw all elements
    for element in &elements {
        draw_element(&painter, element, origin, false);
    }

    // Draw title
    let title = match state.active_converter {
        ConverterType::Buck => "Buck Converter",
        ConverterType::Boost => "Boost Converter",
        ConverterType::VsiSinglePhase => "Single-Phase VSI",
    };
    painter.text(
        Pos2::new(origin.x + 5.0, origin.y + 5.0),
        egui::Align2::LEFT_TOP,
        title,
        egui::TextStyle::Monospace.resolve(ui.style()),
        Color32::GRAY,
    );
}

/// Format a value with appropriate SI prefix.
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
    let decimals = if scaled.abs() >= 100.0 {
        1
    } else if scaled.abs() >= 10.0 {
        2
    } else {
        3
    };
    format!("{:.prec$} {}{}", scaled, prefix, unit, prec = decimals)
}

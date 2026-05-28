use crate::app::state::AppState;
use crate::domain::ConverterType;
use egui::{Slider, Ui};

/// Show parameter sliders and inputs based on the active converter type.
pub fn show_param_panel(ui: &mut Ui, state: &mut AppState) {
    let mut changed = false;

    ui.heading("Input");
    ui.label("Input voltage source");
    if ui
        .add(
            Slider::new(&mut state.params.vin, 1.0..=500.0)
                .text("Vin (V)")
                .suffix(" V"),
        )
        .on_hover_text("DC input voltage to the converter")
        .changed()
    {
        changed = true;
    }

    // Show Vout target for Buck/Boost, hide for VSI (it's derived)
    match state.active_converter {
        ConverterType::Buck | ConverterType::Boost => {
            let conv_name = match state.active_converter {
                ConverterType::Buck => "Buck",
                ConverterType::Boost => "Boost",
                _ => "",
            };
            ui.label(format!(
                "Desired output voltage ({} will auto-calculate duty cycle)",
                conv_name
            ));
            if ui
                .add(
                    Slider::new(&mut state.params.vout_target, 0.5..=500.0)
                        .text("Vout target (V)")
                        .suffix(" V"),
                )
                .on_hover_text("Target output voltage. Duty cycle is calculated automatically.")
                .changed()
            {
                changed = true;
            }
        }
        ConverterType::VsiSinglePhase => {
            ui.label("Modulation index for sine-triangle PWM");
            if ui
                .add(
                    Slider::new(&mut state.params.modulation_index, 0.01..=1.0)
                        .text("Modulation index")
                        .clamping(egui::SliderClamping::Never),
                )
                .on_hover_text("Ratio of reference sine amplitude to carrier amplitude (0..1)")
                .changed()
            {
                changed = true;
            }

            ui.label("Desired AC output frequency");
            if ui
                .add(
                    Slider::new(&mut state.params.output_frequency, 1.0..=1000.0)
                        .text("Output freq (Hz)")
                        .suffix(" Hz"),
                )
                .on_hover_text("Fundamental frequency of the AC output (e.g., 60 Hz)")
                .changed()
            {
                changed = true;
            }
        }
    }

    ui.add_space(8.0);
    ui.heading("Switching");
    ui.label("MOSFET/IGBT switching frequency");
    if ui
        .add(
            Slider::new(&mut state.params.frequency, 100.0..=1_000_000.0)
                .text("Switching freq")
                .logarithmic(true)
                .suffix(" Hz"),
        )
        .on_hover_text(
            "Switching frequency of the power device. Higher f → lower ripple but more switching losses.",
        )
        .changed()
    {
        changed = true;
    }

    ui.add_space(8.0);
    ui.heading("Components");
    ui.label("Inductor value");
    if ui
        .add(
            Slider::new(&mut state.params.inductance, 1e-6..=100e-3)
                .text("Inductance")
                .logarithmic(true)
                .suffix(" H"),
        )
        .on_hover_text("Inductor value. Higher L → lower current ripple but larger/heavier component.")
        .changed()
    {
        changed = true;
    }

    ui.label("Capacitor value");
    if ui
        .add(
            Slider::new(&mut state.params.capacitance, 1e-9..=100e-3)
                .text("Capacitance")
                .logarithmic(true)
                .suffix(" F"),
        )
        .on_hover_text("Output capacitor value. Higher C → lower voltage ripple.")
        .changed()
    {
        changed = true;
    }

    ui.label("Load resistance");
    if ui
        .add(
            Slider::new(&mut state.params.load_resistance, 0.1..=1000.0)
                .text("Load R")
                .logarithmic(true)
                .suffix(" Ω"),
        )
        .on_hover_text("Load resistance. Lower R → higher output current and higher losses.")
        .changed()
    {
        changed = true;
    }

    // Numerical simulation toggle
    ui.add_space(12.0);
    ui.separator();
    ui.checkbox(&mut state.show_numerical_sim, "🧮 Numerical simulation")
        .on_hover_text("Enable RK4 numerical simulation for realistic waveform visualization. Slower but more accurate.");
    if state.show_numerical_sim {
        ui.label("Simulating switching transients...");
    }

    // Trigger recalculation if any parameter changed
    if changed {
        state.recalculate();
    }
}

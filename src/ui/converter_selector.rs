use crate::app::state::AppState;
use crate::domain::ConverterType;
use egui::Ui;

/// Show a row of selectable tabs for choosing the converter type.
pub fn show_converter_selector(ui: &mut Ui, state: &mut AppState) {
    ui.horizontal(|ui| {
        ui.label("Converter:");

        let converters = [
            (ConverterType::Buck, "Buck"),
            (ConverterType::Boost, "Boost"),
            (ConverterType::VsiSinglePhase, "VSI"),
        ];

        for (conv_type, label) in &converters {
            let selected = state.active_converter == *conv_type;
            if ui
                .selectable_label(selected, *label)
                .clicked()
                && !selected
            {
                state.active_converter = *conv_type;
                // Adjust default params based on converter type
                match conv_type {
                    ConverterType::Buck => {
                        state.params.vin = 48.0;
                        state.params.vout_target = 12.0;
                        state.params.frequency = 100_000.0;
                        state.params.inductance = 100e-6;
                        state.params.capacitance = 100e-6;
                        state.params.load_resistance = 10.0;
                    }
                    ConverterType::Boost => {
                        state.params.vin = 12.0;
                        state.params.vout_target = 24.0;
                        state.params.frequency = 100_000.0;
                        state.params.inductance = 100e-6;
                        state.params.capacitance = 100e-6;
                        state.params.load_resistance = 10.0;
                    }
                    ConverterType::VsiSinglePhase => {
                        state.params.vin = 300.0;
                        state.params.vout_target = 240.0;
                        state.params.frequency = 10_000.0;
                        state.params.inductance = 1e-3;
                        state.params.capacitance = 10e-6;
                        state.params.load_resistance = 10.0;
                        state.params.modulation_index = 0.8;
                        state.params.output_frequency = 60.0;
                    }
                }
                state.recalculate();
            }
        }
    });
}

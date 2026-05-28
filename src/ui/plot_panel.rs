use crate::app::state::AppState;
use crate::domain::ConverterType;
use egui::Ui;
use egui_plot::{Legend, Line, Plot, PlotPoints};

/// Show interactive plots for the active converter.
pub fn show_plot_panel(ui: &mut Ui, state: &mut AppState) {
    match state.active_converter {
        ConverterType::Buck | ConverterType::Boost => {
            show_dc_converter_plots(ui, state);
            // If simulation is enabled, overlay simulation waveform
            if state.show_numerical_sim {
                if let Some(ref sim) = state.sim_results {
                    show_simulation_plots(ui, state, sim);
                }
            }
        }
        ConverterType::VsiSinglePhase => {
            show_vsi_plots(ui, state);
            if state.show_numerical_sim {
                if let Some(ref sim) = state.sim_results {
                    show_vsi_simulation_plot(ui, sim);
                }
            }
        }
    }
}

fn show_dc_converter_plots(ui: &mut Ui, state: &AppState) {
    let f = state.params.frequency;
    let t_period = 1.0 / f;
    let n_points = 200;
    let dt = t_period * 3.0 / n_points as f64;

    let mut vout_data: Vec<[f64; 2]> = Vec::with_capacity(n_points);
    let mut il_data: Vec<[f64; 2]> = Vec::with_capacity(n_points);

    let duty = state.params.duty_cycle;
    let vout = state.results.vout;
    let iout = state.results.iout;
    let il_ripple = state.results.il_ripple;

    for i in 0..n_points {
        let t = i as f64 * dt;
        let phase = (t / t_period) % 1.0;

        // Vout waveform (simplified — ideal with ripple)
        let vout_wave = vout
            + (if phase < duty { 1.0 } else { -1.0 }) * state.results.vout_ripple * 0.5;
        vout_data.push([t * 1e6, vout_wave]);

        // Inductor current waveform (triangular approximation)
        let il_instant = if phase < duty {
            iout - il_ripple / 2.0 + (phase / duty) * il_ripple
        } else {
            iout + il_ripple / 2.0 - ((phase - duty) / (1.0 - duty)) * il_ripple
        };
        il_data.push([t * 1e6, il_instant]);
    }

    // Vout Plot
    Plot::new("vout_plot")
        .legend(Legend::default().title("Vout"))
        .height(140.0)
        .width(ui.available_width())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(
                    format!("Vout ({:.2} V)", vout),
                    PlotPoints::from(vout_data),
                )
                .color(egui::Color32::from_rgb(100, 200, 255))
                .width(1.5),
            );
        });

    ui.add_space(4.0);

    // Inductor current plot
    Plot::new("il_plot")
        .legend(Legend::default().title("Inductor Current"))
        .height(140.0)
        .width(ui.available_width())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new(
                    format!("IL ({:.2} A)", iout),
                    PlotPoints::from(il_data),
                )
                .color(egui::Color32::from_rgb(255, 200, 100))
                .width(1.5),
            );
        });
}

/// Show numerical simulation results (overlay on analytical plots).
fn show_simulation_plots(ui: &mut Ui, _state: &AppState, sim: &crate::simulation::integrator::SimulationResult) {
    ui.add_space(4.0);
    ui.label("🧮 Numerical Simulation");

    // Extract iL and vC from simulation result
    let mut il_sim: Vec<[f64; 2]> = Vec::with_capacity(sim.t.len());
    let mut vc_sim: Vec<[f64; 2]> = Vec::with_capacity(sim.t.len());

    for (t, y) in sim.t.iter().zip(sim.y.iter()) {
        il_sim.push([*t * 1e6, y[0]]); // iL in µs scale
        vc_sim.push([*t * 1e6, y[1]]); // vC
    }

    // Vout from simulation
    Plot::new("vout_sim_plot")
        .legend(Legend::default().title("Vc (simulation)"))
        .height(140.0)
        .width(ui.available_width())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Vc (sim)", PlotPoints::from(vc_sim))
                    .color(egui::Color32::from_rgb(50, 255, 100))
                    .width(1.0),
            );
        });

    ui.add_space(4.0);

    // Inductor current from simulation
    Plot::new("il_sim_plot")
        .legend(Legend::default().title("iL (simulation)"))
        .height(140.0)
        .width(ui.available_width())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("iL (sim)", PlotPoints::from(il_sim))
                    .color(egui::Color32::from_rgb(255, 100, 100))
                    .width(1.0),
            );
        });
}

fn show_vsi_plots(ui: &mut Ui, state: &AppState) {
    let f_mod = state.params.output_frequency;
    let f_carrier = state.params.frequency;
    let ma = state.params.modulation_index;
    let vdc = state.params.vin;
    let t_period = 1.0 / f_mod;
    let n_points = 500;
    let dt = t_period * 2.0 / n_points as f64;

    let mut vout_data: Vec<[f64; 2]> = Vec::with_capacity(n_points);
    let mut ref_sine: Vec<[f64; 2]> = Vec::with_capacity(n_points);
    let mut carrier: Vec<[f64; 2]> = Vec::with_capacity(n_points);
    let mut pwm_out: Vec<[f64; 2]> = Vec::with_capacity(n_points);

    let omega_m = 2.0 * std::f64::consts::PI * f_mod;
    let omega_c = 2.0 * std::f64::consts::PI * f_carrier;

    for i in 0..n_points {
        let t = i as f64 * dt;
        let v_sin = ma * (omega_m * t).sin();
        ref_sine.push([t * 1e3, v_sin]);

        let phase_c = (omega_c * t) % (2.0 * std::f64::consts::PI);
        let tri = if phase_c < std::f64::consts::PI {
            phase_c / std::f64::consts::PI * 2.0 - 1.0
        } else {
            1.0 - (phase_c - std::f64::consts::PI) / std::f64::consts::PI * 2.0
        };
        carrier.push([t * 1e3, tri]);

        let pwm_state = if v_sin >= tri { 1.0 } else { -1.0 };
        let v_pwm = pwm_state * vdc / 2.0;
        pwm_out.push([t * 1e3, v_pwm]);

        vout_data.push([t * 1e3, v_sin * vdc / 2.0]);
    }

    // PWM output plot
    Plot::new("vsi_pwm")
        .legend(Legend::default().title("PWM Output"))
        .height(140.0)
        .width(ui.available_width())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Vout (PWM)", PlotPoints::from(pwm_out))
                    .color(egui::Color32::from_rgb(100, 255, 150))
                    .width(1.0),
            );
            plot_ui.line(
                Line::new("Fundamental", PlotPoints::from(vout_data))
                    .color(egui::Color32::from_rgb(255, 100, 100))
                    .width(2.0),
            );
        });

    ui.add_space(4.0);

    // Reference sine + carrier
    Plot::new("vsi_ref")
        .legend(Legend::default().title("Reference & Carrier"))
        .height(120.0)
        .width(ui.available_width())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Reference", PlotPoints::from(ref_sine))
                    .color(egui::Color32::from_rgb(255, 200, 100))
                    .width(1.5),
            );
            plot_ui.line(
                Line::new("Carrier", PlotPoints::from(carrier))
                    .color(egui::Color32::from_rgb(150, 150, 150))
                    .width(0.8),
            );
        });
}

/// Show VSI simulation plot (output current waveform).
fn show_vsi_simulation_plot(ui: &mut Ui, sim: &crate::simulation::integrator::SimulationResult) {
    ui.add_space(4.0);
    ui.label("🧮 Numerical Simulation - Output Current");

    let mut iout_sim: Vec<[f64; 2]> = Vec::with_capacity(sim.t.len());
    for (t, y) in sim.t.iter().zip(sim.y.iter()) {
        iout_sim.push([*t * 1e3, y[0]]); // ms scale
    }

    Plot::new("vsi_iout_sim")
        .legend(Legend::default().title("Iout (simulation)"))
        .height(140.0)
        .width(ui.available_width())
        .show(ui, |plot_ui| {
            plot_ui.line(
                Line::new("Iout (sim)", PlotPoints::from(iout_sim))
                    .color(egui::Color32::from_rgb(255, 150, 50))
                    .width(1.0),
            );
        });
}

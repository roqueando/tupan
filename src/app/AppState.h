#pragma once

#include "domain/Types.h"
#include "domain/converters/Buck.h"
#include "domain/converters/Boost.h"
#include "domain/inverter/VsiSingle.h"
#include "simulation/Integrator.h"
#include "simulation/CircuitOdes.h"
#include "utils/Theme.h"
#include "utils/Formatting.h"
#include <string>
#include <optional>
#include <functional>

/// Main application state — single source of truth.
class AppState {
public:
    // ── State fields ──
    ConverterType active_converter = ConverterType::Buck;
    ConverterParams params;
    ConverterResults results;
    std::optional<SimulationResult> sim_results;

    // UI state
    Theme theme = Theme::Dark;
    bool show_numerical_sim = false;
    bool show_schematic = true;
    std::string status_message = "ready";

    // ── Methods ──

    /// Recalculate all analytical results based on current params and converter type.
    void recalculate() {
        switch (active_converter) {
            case ConverterType::Buck:
                results = buck::calculate(params);
                break;
            case ConverterType::Boost:
                results = boost_converter::calculate(params);
                break;
            case ConverterType::VsiSinglePhase:
                results = vsi_single::calculate(params, true);
                break;
        }

        if (show_numerical_sim) {
            run_simulation();
        }
    }

    /// Run numerical simulation (RK4) for active converter.
    void run_simulation() {
        using namespace integrator;

        switch (active_converter) {
            case ConverterType::Buck: {
                auto ode = circuit_odes::BuckOde::from_params(params);
                double vout_est = params.vin * params.duty_cycle;
                double iout_est = (params.load_resistance > 0.0) ? vout_est / params.load_resistance : 0.0;
                std::vector<double> y0 = {iout_est, vout_est};

                DerivFn f = [ode](double t, const std::vector<double>& y) {
                    return ode.derivatives(t, y);
                };
                sim_results = integrate_fixed(f, y0, 0.0, 0.005, 1e-8, 5000);
                break;
            }
            case ConverterType::Boost: {
                auto ode = circuit_odes::BoostOde::from_params(params);
                double vout_est = (1.0 - params.duty_cycle > 0.01) ? params.vin / (1.0 - params.duty_cycle) : 0.0;
                double iout_est = (params.load_resistance > 0.0) ? vout_est / params.load_resistance : 0.0;
                double iin_est = (1.0 - params.duty_cycle > 0.01) ? iout_est / (1.0 - params.duty_cycle) : 0.0;
                std::vector<double> y0 = {iin_est, vout_est};

                DerivFn f = [ode](double t, const std::vector<double>& y) {
                    return ode.derivatives(t, y);
                };
                sim_results = integrate_fixed(f, y0, 0.0, 0.005, 1e-8, 5000);
                break;
            }
            case ConverterType::VsiSinglePhase: {
                auto ode = circuit_odes::VsiOde::from_params(params);
                std::vector<double> y0 = {0.0};

                DerivFn f = [ode](double t, const std::vector<double>& y) {
                    return ode.derivatives(t, y);
                };
                sim_results = integrate_fixed(f, y0, 0.0, 0.05, 1e-6, 10000);
                break;
            }
        }
    }

    /// Build ComponentValues from current state for schematic annotation.
    ComponentValues get_component_values() const {
        ComponentValues v;
        v.vin = formatting::format_value(params.vin, "V");
        v.vout = formatting::format_value(results.vout, "V");
        v.inductance = formatting::format_value(params.inductance, "H");
        v.capacitance = formatting::format_value(params.capacitance, "F");
        v.load = formatting::format_value(params.load_resistance, "Ohm");
        v.frequency = formatting::format_value(params.frequency, "Hz");
        v.duty_cycle = formatting::format_value(params.duty_cycle * 100.0, "%");
        return v;
    }
};

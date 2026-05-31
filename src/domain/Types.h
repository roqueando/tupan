#pragma once

#include <optional>
#include <string>
#include <vector>

// ── Converter Type ─────────────────────────────────────────────────────

enum class ConverterType {
    Buck,
    Boost,
    VsiSinglePhase
};

inline const char* converter_type_name(ConverterType t) {
    switch (t) {
        case ConverterType::Buck:           return "Buck Converter";
        case ConverterType::Boost:          return "Boost Converter";
        case ConverterType::VsiSinglePhase: return "VSI Single-Phase";
    }
    return "Unknown";
}

// ── Converter Parameters ──────────────────────────────────────────────

struct ConverterParams {
    // Input
    double vin = 48.0;          // V
    double vout_target = 12.0;  // V (target)

    // Switching
    double frequency = 100'000.0; // Hz
    double duty_cycle = 0.5;      // 0..1

    // Components
    double inductance = 100e-6;     // H
    double capacitance = 100e-6;    // F
    double load_resistance = 10.0;  // Ohm

    // Inverter-specific
    double modulation_index = 0.8;  // 0..1
    double output_frequency = 60.0; // Hz
};

// ── Converter Results ─────────────────────────────────────────────────

struct ConverterResults {
    // Voltage / Current
    double vout = 0.0;            // Average output voltage (V)
    double iout = 0.0;            // Average output current (A)
    double iin = 0.0;             // Average input current (A)

    // Ripple
    double vout_ripple = 0.0;     // Output voltage ripple (Vpp)
    double il_ripple = 0.0;       // Inductor current ripple (App)

    // Losses & Efficiency
    double conduction_losses = 0.0;
    double switching_losses = 0.0;
    double efficiency = 0.0;      // 0..1

    // Inverter-specific
    std::optional<double> thd;
    std::optional<double> rms_output;
    std::optional<double> fundamental_amplitude;

    static ConverterResults zero() {
        return ConverterResults{};
    }
};

// ── Simulation Result ──────────────────────────────────────────────────

struct SimulationResult {
    std::vector<double> t;           // Time points
    std::vector<std::vector<double>> y;  // State vectors at each time
};

// ── Theme ─────────────────────────────────────────────────────────────

enum class Theme { Dark, Light };

// ── Canvas State Types ────────────────────────────────────────────────

struct Pos {
    float x = 0.0f;
    float y = 0.0f;
    Pos() = default;
    Pos(float x, float y) : x(x), y(y) {}
};

struct ComponentValues {
    std::string vin;
    std::string vout;
    std::string inductance;
    std::string capacitance;
    std::string load;
    std::string frequency;
    std::string duty_cycle;
};

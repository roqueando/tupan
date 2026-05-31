#pragma once

#include <optional>
#include <string>
#include <vector>

// ── Converter Type ─────────────────────────────────────────────────────

enum class ConverterType {
    Buck, Boost, VsiSinglePhase
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
    double vin = 48.0;
    double vout_target = 12.0;
    double frequency = 100'000.0;
    double duty_cycle = 0.5;
    double inductance = 100e-6;
    double capacitance = 100e-6;
    double load_resistance = 10.0;
    double modulation_index = 0.8;
    double output_frequency = 60.0;
};

// ── Converter Results ─────────────────────────────────────────────────

struct ConverterResults {
    double vout = 0.0;
    double iout = 0.0;
    double iin = 0.0;
    double vout_ripple = 0.0;
    double il_ripple = 0.0;
    double conduction_losses = 0.0;
    double switching_losses = 0.0;
    double efficiency = 0.0;
    std::optional<double> thd;
    std::optional<double> rms_output;
    std::optional<double> fundamental_amplitude;
};

// ── Simulation Result ─────────────────────────────────────────────────

struct SimulationResult {
    std::vector<double> t;
    std::vector<std::vector<double>> y;
};

// ── Theme ─────────────────────────────────────────────────────────────

enum class Theme { Dark, Light };

// ── Canvas Position ───────────────────────────────────────────────────

struct Pos {
    float x = 0.0f;
    float y = 0.0f;
    Pos() = default;
    Pos(float x, float y) : x(x), y(y) {}
};

// ── Component Values (for schematic annotation) ───────────────────────

struct ComponentValues {
    std::string vin;
    std::string vout;
    std::string inductance;
    std::string capacitance;
    std::string load;
    std::string frequency;
    std::string duty_cycle;
};

// ── Canvas Component Types ────────────────────────────────────────────

enum class CanvasComponentType {
    Vin, Vout, DutyCycle, Frequency, DeltaIl, IoutMax, DeltaVo,
    Inductor, Capacitor, Plot
};

inline const char* canvas_type_name(CanvasComponentType t) {
    switch (t) {
        case CanvasComponentType::Vin:       return "Vin";
        case CanvasComponentType::Vout:      return "Vout";
        case CanvasComponentType::DutyCycle: return "Duty Cycle";
        case CanvasComponentType::Frequency: return "Frequency";
        case CanvasComponentType::DeltaIl:   return "\xce\x94iL"; // ΔiL in UTF-8
        case CanvasComponentType::IoutMax:   return "Iout,max";
        case CanvasComponentType::DeltaVo:   return "\xce\x94Vo"; // ΔVo in UTF-8
        case CanvasComponentType::Inductor:  return "Inductor (L)";
        case CanvasComponentType::Capacitor: return "Capacitor (C)";
        case CanvasComponentType::Plot:      return "Plot";
    }
    return "?";
}

inline const char* canvas_type_icon(CanvasComponentType t) {
    switch (t) {
        case CanvasComponentType::Vin:       return "\xe2\x9a\xa1"; // ⚡
        case CanvasComponentType::Vout:      return "\xf0\x9f\x94\x8c"; // 🔌
        case CanvasComponentType::DutyCycle: return "\xe3\x80\xb0"; // 〰
        case CanvasComponentType::Frequency: return "\xf0\x9f\x93\xa1"; // 📡
        case CanvasComponentType::DeltaIl:   return "\xf0\x9f\x93\x89"; // 📉
        case CanvasComponentType::IoutMax:   return "\xf0\x9f\x92\xa7"; // 💧
        case CanvasComponentType::DeltaVo:   return "\xf0\x9f\x93\x8a"; // 📊
        case CanvasComponentType::Inductor:  return "\xe3\x80\xb0"; // 〰
        case CanvasComponentType::Capacitor: return "\xe2\x80\x96\xe2\x80\x96"; // ‖‖
        case CanvasComponentType::Plot:      return "\xf0\x9f\x93\x88"; // 📈
    }
    return "?";
}

inline const char* canvas_type_unit(CanvasComponentType t) {
    switch (t) {
        case CanvasComponentType::Vin:       return "V";
        case CanvasComponentType::Vout:      return "V";
        case CanvasComponentType::DutyCycle: return "%";
        case CanvasComponentType::Frequency: return "Hz";
        case CanvasComponentType::DeltaIl:   return "%";
        case CanvasComponentType::IoutMax:   return "A";
        case CanvasComponentType::DeltaVo:   return "%";
        case CanvasComponentType::Inductor:  return "H";
        case CanvasComponentType::Capacitor: return "F";
        case CanvasComponentType::Plot:      return "";
    }
    return "";
}

inline bool is_editable(CanvasComponentType t) {
    return t == CanvasComponentType::Vin || t == CanvasComponentType::Vout ||
           t == CanvasComponentType::DutyCycle || t == CanvasComponentType::Frequency ||
           t == CanvasComponentType::DeltaIl || t == CanvasComponentType::IoutMax ||
           t == CanvasComponentType::DeltaVo;
}

inline bool is_computed(CanvasComponentType t) {
    return t == CanvasComponentType::Inductor || t == CanvasComponentType::Capacitor;
}

inline bool is_plot(CanvasComponentType t) {
    return t == CanvasComponentType::Plot;
}

// ── Placed Component ──────────────────────────────────────────────────

struct PlacedComponent {
    uint64_t id = 0;
    CanvasComponentType component_type = CanvasComponentType::Vin;
    Pos pos;
};

// ── Shared Parameters ─────────────────────────────────────────────────

struct SharedParams {
    double vin = 48.0;
    double vout = 12.0;
    double duty_cycle = 0.25;
    double frequency = 100'000.0;
    double delta_il = 0.3;
    double iout_max = 5.0;
    double delta_vo = 0.01;

    double calc_inductance() const {
        if (delta_il <= 0.0 || frequency <= 0.0) return 0.0;
        double dil_amps = delta_il * iout_max;
        if (dil_amps <= 0.0) return 0.0;
        return (vout * (1.0 - duty_cycle)) / (dil_amps * frequency);
    }

    double calc_delta_il_amps() const { return delta_il * iout_max; }

    double calc_capacitance() const {
        double l = calc_inductance();
        if (l <= 0.0 || delta_vo <= 0.0 || frequency <= 0.0) return 0.0;
        return (1.0 - duty_cycle) / (8.0 * l * delta_vo * frequency * frequency);
    }
};

// ── Canvas State ──────────────────────────────────────────────────────

struct CanvasState {
    std::vector<PlacedComponent> placed_components;
    SharedParams shared_params;
    float pan_x = 0.0f, pan_y = 0.0f;
    float zoom = 1.0f;
    uint64_t next_id = 1;
    std::optional<size_t> selected_index;
    std::optional<CanvasComponentType> palette_selection;

    void clear() { placed_components.clear(); selected_index.reset(); }

    void delete_selected() {
        if (selected_index && *selected_index < placed_components.size()) {
            placed_components.erase(placed_components.begin() + *selected_index);
            selected_index.reset();
        }
    }

    void place_component(CanvasComponentType type, const Pos& pos) {
        placed_components.push_back({next_id++, type, pos});
    }

    double get_value(CanvasComponentType ct) const {
        switch (ct) {
            case CanvasComponentType::Vin:       return shared_params.vin;
            case CanvasComponentType::Vout:      return shared_params.vout;
            case CanvasComponentType::DutyCycle: return shared_params.duty_cycle * 100.0;
            case CanvasComponentType::Frequency: return shared_params.frequency;
            case CanvasComponentType::DeltaIl:   return shared_params.delta_il * 100.0;
            case CanvasComponentType::IoutMax:   return shared_params.iout_max;
            case CanvasComponentType::DeltaVo:   return shared_params.delta_vo * 100.0;
            case CanvasComponentType::Inductor:  return shared_params.calc_inductance();
            case CanvasComponentType::Capacitor: return shared_params.calc_capacitance();
            case CanvasComponentType::Plot:      return 0.0;
        }
        return 0.0;
    }

    bool set_value(CanvasComponentType ct, double value) {
        switch (ct) {
            case CanvasComponentType::Vin:
                if (std::abs(shared_params.vin - value) > 1e-12) { shared_params.vin = value; return true; }
                break;
            case CanvasComponentType::Vout:
                if (std::abs(shared_params.vout - value) > 1e-12) {
                    shared_params.vout = value;
                    if (shared_params.vin > 0.0)
                        shared_params.duty_cycle = std::clamp(value / shared_params.vin, 0.0, 1.0);
                    return true;
                }
                break;
            case CanvasComponentType::DutyCycle: {
                double dc = std::clamp(value / 100.0, 0.0, 1.0);
                if (std::abs(shared_params.duty_cycle - dc) > 1e-12) {
                    shared_params.duty_cycle = dc;
                    shared_params.vout = shared_params.vin * dc;
                    return true;
                }
                break;
            }
            case CanvasComponentType::Frequency:
                if (std::abs(shared_params.frequency - value) > 1e-12 && value > 0.0) { shared_params.frequency = value; return true; }
                break;
            case CanvasComponentType::DeltaIl: {
                double pct = std::max(value / 100.0, 0.001);
                if (std::abs(shared_params.delta_il - pct) > 1e-12) { shared_params.delta_il = pct; return true; }
                break;
            }
            case CanvasComponentType::IoutMax:
                if (std::abs(shared_params.iout_max - value) > 1e-12) { shared_params.iout_max = value; return true; }
                break;
            case CanvasComponentType::DeltaVo: {
                double pct = std::max(value / 100.0, 0.0001);
                if (std::abs(shared_params.delta_vo - pct) > 1e-12) { shared_params.delta_vo = pct; return true; }
                break;
            }
            default: break;
        }
        return false;
    }
};

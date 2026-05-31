#include "CircuitOdes.h"
#include "domain/converters/Common.h"
#include <cmath>
#include <vector>

namespace circuit_odes {

// ── BuckOde ───────────────────────────────────────────────────────────

BuckOde BuckOde::from_params(const ConverterParams& params) {
    return BuckOde{
        params.vin, params.inductance, params.capacitance,
        params.load_resistance, params.frequency, params.duty_cycle
    };
}

double buck_switching(double t, double frequency, double duty) {
    double period = 1.0 / frequency;
    double phase = std::fmod(t, period) / period;
    return (phase < duty) ? 1.0 : 0.0;
}

std::vector<double> BuckOde::derivatives(double t, const std::vector<double>& y) const {
    double il = y[0];
    double vc = y[1];
    double s = buck_switching(t, frequency, duty);

    double dil_dt = (s * vin - vc) / l;
    double dvc_dt = (il - vc / r) / c;

    return {dil_dt, dvc_dt};
}

// ── BoostOde ──────────────────────────────────────────────────────────

BoostOde BoostOde::from_params(const ConverterParams& params) {
    return BoostOde{
        params.vin, params.inductance, params.capacitance,
        params.load_resistance, params.frequency, params.duty_cycle
    };
}

std::vector<double> BoostOde::derivatives(double t, const std::vector<double>& y) const {
    double il = y[0];
    double vc = y[1];
    double s = buck_switching(t, frequency, duty);

    double dil_dt = (vin - (1.0 - s) * vc) / l;
    double dvc_dt = ((1.0 - s) * il - vc / r) / c;

    return {dil_dt, dvc_dt};
}

// ── VsiOde ────────────────────────────────────────────────────────────

VsiOde VsiOde::from_params(const ConverterParams& params) {
    return VsiOde{
        params.vin, params.load_resistance, params.inductance,
        params.frequency, params.output_frequency, params.modulation_index
    };
}

double VsiOde::pwm_voltage(double t) const {
    double omega_m = 2.0 * converter_common::PI * mod_freq;
    double omega_c = 2.0 * converter_common::PI * carrier_freq;

    double v_ref = ma * std::sin(omega_m * t);
    double phase_c = std::fmod(omega_c * t, 2.0 * converter_common::PI);
    double triangle;
    if (phase_c < converter_common::PI) {
        triangle = (phase_c / converter_common::PI) * 2.0 - 1.0;
    } else {
        triangle = 1.0 - (phase_c - converter_common::PI) / converter_common::PI * 2.0;
    }

    return (v_ref >= triangle) ? (vdc / 2.0) : (-vdc / 2.0);
}

std::vector<double> VsiOde::derivatives(double t, const std::vector<double>& y) const {
    double i = y[0];
    double v_pwm = pwm_voltage(t);
    double di_dt = (v_pwm - r_load * i) / l_load;
    return {di_dt};
}

} // namespace circuit_odes

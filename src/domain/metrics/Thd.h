#pragma once

#include <vector>

namespace thd {

/// Calculate THD from harmonic amplitudes.
double thd_from_harmonics(const std::vector<double>& harmonics, double fundamental);

/// Theoretical THD for a bipolar PWM sine wave with given modulation index.
double pwm_thd_approximate(double modulation_index, bool is_bipolar);

/// Calculate the fundamental component amplitude for a PWM inverter.
double fundamental_amplitude(double modulation_index, double vdc, bool is_full_bridge);

/// Calculate RMS output voltage for a PWM inverter.
double rms_output_voltage(double modulation_index, double vdc, bool is_full_bridge);

} // namespace thd

#pragma once

#include "domain/Types.h"

namespace pwm {

/// Generate a sine-triangle PWM switching function.
/// Returns a vector of (time, state) pairs where state is 1.0 or -1.0.
std::vector<std::pair<double, double>> generate_pwm(
    double ma, double modulation_freq, double carrier_freq,
    double num_periods, double dt);

/// Calculate the duty cycle for a given reference angle in sine PWM.
double duty_cycle_at_time(double ma, double omega_m, double t);

/// Calculate the frequency modulation ratio.
double frequency_modulation_ratio(double carrier_freq, double modulation_freq);

} // namespace pwm

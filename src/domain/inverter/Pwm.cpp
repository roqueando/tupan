#include "Pwm.h"
#include "domain/converters/Common.h"
#include <cmath>
#include <vector>

namespace pwm {

std::vector<std::pair<double, double>> generate_pwm(
    double ma, double modulation_freq, double carrier_freq,
    double num_periods, double dt)
{
    ma = converter_common::clamp(ma, 0.0, 1.0);
    double t_total = num_periods / modulation_freq;
    int n_points = static_cast<int>(t_total / dt);
    double omega_m = converter_common::angular_frequency(modulation_freq);
    double omega_c = converter_common::angular_frequency(carrier_freq);

    std::vector<std::pair<double, double>> samples;
    samples.reserve(n_points);

    for (int i = 0; i < n_points; ++i) {
        double t = i * dt;
        if (t > t_total) break;

        double v_mod = ma * std::sin(omega_m * t);
        double phase_c = std::fmod(omega_c * t, 2.0 * converter_common::PI);
        double triangle;
        if (phase_c < converter_common::PI) {
            triangle = (phase_c / converter_common::PI) * 2.0 - 1.0;
        } else {
            double fall_phase = phase_c - converter_common::PI;
            triangle = 1.0 - (fall_phase / converter_common::PI) * 2.0;
        }

        double state = (v_mod >= triangle) ? 1.0 : -1.0;
        samples.emplace_back(t, state);
    }

    return samples;
}

double duty_cycle_at_time(double ma, double omega_m, double t) {
    return converter_common::clamp(0.5 * (1.0 + ma * std::sin(omega_m * t)), 0.0, 1.0);
}

double frequency_modulation_ratio(double carrier_freq, double modulation_freq) {
    if (modulation_freq <= 0.0) return 0.0;
    return carrier_freq / modulation_freq;
}

} // namespace pwm

#include "Thd.h"
#include <cmath>
#include <vector>

namespace thd {

double thd_from_harmonics(const std::vector<double>& harmonics, double fundamental) {
    if (std::abs(fundamental) <= 1e-12) return 0.0;
    double sum_sq = 0.0;
    for (double h : harmonics) sum_sq += h * h;
    return std::sqrt(sum_sq) / std::abs(fundamental);
}

double pwm_thd_approximate(double modulation_index, bool is_bipolar) {
    double ma = std::clamp(modulation_index, 0.0, 1.0);

    if (is_bipolar) {
        if (ma < 0.01) return 10.0;
        double thd_sq = std::pow(1.12 / ma, 2) - 1.0;
        return std::min(std::sqrt(thd_sq), 10.0);
    } else {
        if (ma < 0.01) return 5.0;
        double thd_sq = std::pow(0.6 / ma, 2) - 1.0;
        return std::min(std::sqrt(thd_sq), 5.0);
    }
}

double fundamental_amplitude(double modulation_index, double vdc, bool is_full_bridge) {
    double ma = std::clamp(modulation_index, 0.0, 1.0);
    if (is_full_bridge) return ma * vdc;
    else return ma * vdc / 2.0;
}

double rms_output_voltage(double modulation_index, double vdc, bool is_full_bridge) {
    double v1 = fundamental_amplitude(modulation_index, vdc, is_full_bridge);
    return v1 / std::sqrt(2.0);
}

} // namespace thd
